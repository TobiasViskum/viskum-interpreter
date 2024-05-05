[bits 64]
global _start

section .data
    num db '4', 10  ; ASCII value for '4' and newline

section .text
_start:
    ; write syscall
    mov rax, 1      ; syscall number for sys_write
    mov rdi, 1      ; file descriptor 1 is stdout
    mov rsi, num    ; pointer to message to write
    mov rdx, 2      ; message length
    syscall

    ; exit syscall
    mov rax, 60     ; syscall number for sys_exit
    xor rdi, rdi    ; exit code 0
    syscall
