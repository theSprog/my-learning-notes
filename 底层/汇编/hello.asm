section .text
	global _start2
_start2:
	mov rax, 1		; write(
	mov rdi, 1		;	stdout
	mov rsi, msg 	;	msg
	msg_len equ 12
	mov rdx, msg_len;	12
	syscall			;)

	mov rax, 60		; exit(
	mov rdi, 0		; 0
	syscall			;)

section .data
	msg db "hello world", 10	; 10 是换行


section .bss