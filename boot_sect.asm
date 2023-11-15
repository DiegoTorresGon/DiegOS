; This is a program that tries to print X by addressing in several ways.

[org 0x7c00] ; this tells the assembler our code will be stored starting in address 0x7c00

mov ah, 	0x0e 			; This is for the bios routine that prints characters, you have to trigger interrup 0x10 and have 0x0eXX in the
							; ax register, where XX is the ascii code of the letter being used.

mov al, '1'
int 0x10
mov al, ':'
int 0x10

mov al, the_secret
int 0x10

mov al, 0x0d
int 0x10
mov al, 0x0a
int 0x10

mov al, '2'
int 0x10
mov al, ':'
int 0x10

mov al, [the_secret]
int 0x10

mov al, 0x0d
int 0x10
mov al, 0x0a
int 0x10

mov al, '3'
int 0x10
mov al, ':'
int 0x10

mov bx, the_secret
add bx, 0x7c00 ; 0x7c00 is the adress where this boot sector starts.
mov al, [bx]
int 0x10

mov al, 0x0d
int 0x10
mov al, 0x0a
int 0x10

mov al, '4'
int 0x10
mov al, ':'
int 0x10

mov al, [0x7c5d]
int 0x10

mov al, 0x0d
int 0x10
mov al, 0x0a
int 0x10

jmp $

the_secret:
	db "X"

times 510 - ($ - $$) db 0
dw 0xaa55
