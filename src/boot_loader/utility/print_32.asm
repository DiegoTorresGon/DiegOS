[bits 32]

; Utitlity function to print a str. Receives address of string in ebx register.
print32_str:
	pusha
	mov edx, VIDEO_MEMORY
print32_loop:
	mov byte al, [ebx] ;printing char to al
	mov ah, WHITE_ON_BLACK

	mov [edx], ax ;move char to vid. mem.
	add ebx, 1 ; go to next char
	add edx, 2 ; next video space	
	
	cmp byte [ebx], 0x0
	jne print32_loop

	popa
	ret

println32:
	call print32_str 
	ret

; Print a 16-bit number in hex representation.
; arg dx register
print32_hex:
	pusha
	mov al, dl
	call get_chars_8_bit32
	mov word [HEX_OUT + 4], ax
	mov al, dh
	call get_chars_8_bit32 
	mov word [HEX_OUT + 2], ax
	mov ebx, HEX_OUT
	call print32_str 
	popa
	ret

; arg al value
; return value's hex representation in ax 
get_chars_8_bit32:
	mov bx, ax
	and ebx, 0x0000000F
	mov byte ah, [HEX_VALUES + ebx] 
	mov bx, ax
	and ebx, 0x000000F0
	shr ebx, 4
	mov byte al, [HEX_VALUES + ebx]
	ret

VIDEO_MEMORY: equ 0xb8000
WHITE_ON_BLACK: equ 0x0f
