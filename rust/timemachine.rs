#[cfg(test)]
mod timemachine {
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::task::Context;
    use std::task::Poll;
    use std::task::Waker;
    use std::time::Duration;

    use futures::Future;
    use lazy_static::lazy_static;

    struct SharedState {
        id: u64,
        wakeup_time: Duration,
        /// Whether or not the sleep time has elapsed
        completed: bool,
    
        /// The waker for the task that `TimerFuture` is running on.
        /// The thread can use this after setting `completed = true` to tell
        /// `TimerFuture`'s task to wake up, see that `completed = true`, and
        /// move forward.
        waker: Option<Waker>,
    }

    lazy_static! {
        static ref SLEEP_ID: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
        static ref NOW: std::sync::Mutex<Duration> = std::sync::Mutex::new(Duration::from_secs(0));
        static ref WAKEUPS: std::sync::Mutex<Vec<Arc<Mutex<SharedState>>>> = std::sync::Mutex::new(Vec::new());
    }

    struct Internal {
        id: u64,
    }

    impl Drop for Internal {
        fn drop(&mut self) {
            log::info!("[{}] drop", self.id);

            let mut wakeups = WAKEUPS.lock().unwrap();
            wakeups.retain(|p| p.lock().unwrap().id != self.id);
        }
    }

    pub struct Sleep {
        shared_state: Arc<Mutex<SharedState>>,
        internal: Arc<Internal>
    }

    impl Clone for Sleep {
        fn clone(&self) -> Self {
            Self {
                shared_state: self.shared_state.clone(),
                internal: self.internal.clone(),
            }
        }
    }

    impl Future for Sleep {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            log::info!("[{}] poll", self.internal.id);

            let mut shared_state = self.shared_state.lock().unwrap();

            if shared_state.completed {
                log::info!("[{}] poll: completed", self.internal.id);

                Poll::Ready(())
            } else {
                log::info!("[{}] poll: pending", self.internal.id);

                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }

    pub fn sleep(duration: Duration) -> Sleep {
        let now = NOW.lock().unwrap();
        let wakeup_time = *now + duration;
        let mut curr_id = SLEEP_ID.lock().unwrap();
        let id = *curr_id;
        *curr_id += 1;

        log::info!("[{}] sleep now: {:?} duration: {:?} wakeup_time: {:?}", id, *now, duration, wakeup_time);

        let internal = Arc::new(Internal { id });

        let state = Arc::new(Mutex::new(SharedState {
            id,
            wakeup_time: wakeup_time,
            completed: false,
            waker: None,
        }));

        {
            let mut wakeups = WAKEUPS.lock().unwrap();
            
            if wakeups.len() == 0 {
                wakeups.push(state.clone());
            } else {
                for (index, wakeup) in wakeups.iter().enumerate() {
                    let time = {
                        let wakeup = wakeup.lock().unwrap();
                        wakeup.wakeup_time
                    };

                    if wakeup_time < time {
                        wakeups.insert(index, state.clone());
                        break;
                    }
                }
            }
        }

        Sleep {
            shared_state: state,
            internal,
        }
    }

    pub fn go_forward(duration: Duration) {
        let mut now = NOW.lock().unwrap();

        log::info!("going forward now: {:?} duration: {:?}", *now, duration);

        *now += duration;

        log::info!("new now {:?}", *now);
        

        let mut wakeups = WAKEUPS.lock().unwrap();

        log::info!("wakeups aquired");

        let mut remove = vec![];

        for (index, wakeup) in wakeups.iter().enumerate() {
            log::info!("checking wakeup {}", index);

            let mut wakeup = wakeup.lock().unwrap();

            log::info!("[{}] check wakeup {:?}", wakeup.id, wakeup.wakeup_time);

            if wakeup.wakeup_time <= *now {
                wakeup.completed = true;
                if let Some(waker) = wakeup.waker.take() {
                    log::info!("[{}] waking {} now {:?} wakeup_time {:?}", wakeup.id, index, *now, wakeup.wakeup_time);

                    waker.wake();
                    remove.push(index);
                }
            }
        }

        for index in remove.iter().rev() {
            wakeups.remove(*index);
        }
    }
}

#[cfg(not(test))]
mod timemachine {
    use std::time::Duration;
    use tokio::time::Sleep;

    pub async fn sleep(duration: Duration) -> Sleep {
        tokio::time::sleep(duration)
    }
}

pub use timemachine::*;