; This is intended to do a memory map using int15, eax=e820

n_entries equ 0x9000
SMAP equ 0x0534D4150

mapping_routine:
	mov di , 0x9004
	xor ebx, ebx				; ebx should be set to 0
	xor bp, bp 					; Reset bp so it serves as entry count
	mov edx, SMAP				; Magic value "SMAP"
	mov eax, 0xE820				; mem map routine
	mov [es:di + 20], dword 1 	; force a valid ACPI
	mov ecx, 24 				; ask routing for 24 bytes
	int 0x15
	
	jc .failed
	mov edx, SMAP				; In case ebx was trashed
	cmp eax, edx				; On success, eax should be SMAP
	jne .failed
	test ebx, ebx				; ebx = 0 implies list has 1 entry (worthless)
	je .failed
	jmp .jmpin

.e820lp:
	mov eax, 0xe820 			; eax, ecx get trashed on every int call.
	mov [es:di + 20], dword 1
	mov ecx, 24
	int 0x15
	jc .e820f
	mov edx, SMAP
.jmpin:
	jcxz .skipent
	cmp cl, 20
	jbe .notext
.notext:
	mov ecx, [es:di + 8] 		; get lower uint32_t of memory region length
	or ecx, [es:di + 12]		; get upper part to test with 0
	jz .skipent					; if length if 0, skip entry
	inc bp						; entry is good. Move to next entry.
	add di, 24
.skipent:
	test ebx, ebx				; if ebx = 0, list is complete
	jne .e820lp
.e820f:
	mov [n_entries], bp
	clc
	mov bx, SUCCESS
	call println32
	ret

.failed:
	mov bx, UNSUPPORTED
	call println32

	jmp $

UNSUPPORTED:
	db "Unsupported", 0
SUCCESS:
	db "YI!", 0
