FROM ubuntu

RUN apt-get update
RUN apt-get install -y vim git curl iverilog tmux

WORKDIR /root

RUN curl -LO https://golang.org/dl/go1.15.6.linux-amd64.tar.gz

RUN tar -C /usr/local -xzf go1.15.6.linux-amd64.tar.gz

RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage
RUN chmod u+x nvim.appimage

RUN ./nvim.appimage --appimage-extract
RUN ./squashfs-root/AppRun --version

# Optional: exposing nvim globally 
RUN cp -R /root/squashfs-root /squashfs-root && ln -s /squashfs-root/AppRun /usr/bin/nvim
# nvim

# RUN ./nvim.appimage

RUN sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs \
       https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'

ADD init.vim /root/.config/nvim/init.vim
# ADD .vimrc /root/.vimrc

ADD .bashrc /root/.bashrc