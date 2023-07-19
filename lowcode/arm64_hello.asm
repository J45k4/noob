.global _start
.align 2

.text
_start:
    // Write "Hello, World!" to standard output (file descriptor 1)
    mov    x0,  1             // file descriptor 1
    adrp   x1,  message@PAGE        // address of the message
    mov    x2,  13            // length of the message
    mov    x16, 4            // syscall number for write
    svc    0x80                 // make the system call

    // Exit the program
    mov    x0,  0             // exit code 0
    mov    x16, 1            // syscall number for exit
    svc    0                 // make the system call

.data
message:
    .ascii "Hello, World!\n"
