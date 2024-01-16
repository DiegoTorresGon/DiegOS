[bits 16]

; Utitlity function to print a str. Receives address of string in bx register.
print_str:
	pusha
	mov ah, 0x0e
print_loop:
	mov byte al, [bx]
	int 0x10
	inc word bx
	cmp byte [bx], 0x0
	jne print_loop

	popa
	ret

println:
	pusha
	mov ah, 0x0e
	call print_str
	mov al, 0x0d
	int 0x10
	mov al, 0x0a
	int 0x10
	popa
	ret
	
println_hex:
	pusha
	call print_hex
	mov ah, 0x0e
	mov al, 0x0d
	int 0x10
	mov al, 0x0a
	int 0x10
	popa
	ret

; Print a 16-bit number in hex representation.
; arg dx register
print_hex:
	pusha
	mov al, dl
	call get_chars_8_bit
	mov word [HEX_OUT + 4], ax
	mov al, dh
	call get_chars_8_bit 
	mov word [HEX_OUT + 2], ax
	mov bx, HEX_OUT
	call print_str 
	popa
	ret
	


; arg al value
; return value's hex representation in ax 
get_chars_8_bit:
	mov bx, ax
	and bx, 0x000F
	mov byte ah, [HEX_VALUES + bx] 
	mov bx, ax
	and bx, 0x00F0
	shr bx, 4
	mov byte al, [HEX_VALUES + bx]
	ret

; global variables
HEX_OUT: db "0x0000", 0
HEX_VALUES: db "0123456789abcdef"
