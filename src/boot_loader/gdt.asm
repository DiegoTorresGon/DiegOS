; This file defines the GDT. This is enabling us to enter 32-bit protected mode.
gdt_start:

gdt_null:  ; This is a mandatory null descriptor.
	dd 0x0	; This is so the cpu know if we forgot to set
	dd 0x0 ; the segment register.
			; In that case we would access this null descriptor.

; We are using a simple flat model.
; This means data and code segment use the same physical space.
; Because of this, the code segment is not protected from the data.

;This will have an offset of 8 bytes from the start as the previous
;descriptor if 2 double words (8 bytes)
gdt_code: ; This is the code segment descriptor
	; base = 0x0, limit = 0xffff,
	; 1st flags: (present)1 (privilege)00 (descriptor type)1 => 1001b
	; type flags: (code)1 (conforming)0 (readable)1 (accesed)0 => 1010b
	; 2nd flags: (granularity)1 (32-bit default)1 (64-bit segment)0 (AVL)0 => 1100b
	dw 0xffff ; limit (bits 15-0)
	dw 0x0000 ; base (bits 15-0) ; This is part of the mechanism for paging, this
	db 0x00 ; base (bits 23-16)
	db 10011010b ; 1st flags, type flags
	db 11001111b ; 2nd flags, limit (bits 16-19)
	db 0x00 ; base (bits 31-24)

gdt_data: ; This is the data segment descriptor
	; Same as code but type flags change.
	; type flags: (code)0 (expand down)0 (readable)1 (accesed)0 => 0010b
	dw 0xffff ; limit (bits 15-0)
	dw 0x0000 ; base (bits 15-0) ; This is part of the mechanism for paging, this
	db 0x00 ; base (bits 23-16)
	db 10010010b ; 1st flags, type flags
	db 11001111b ; 2nd flags, limit (bits 16-19)
	db 0x00 ; base (bits 31-24)
	
gdt_end: ; We put a label here so the compiler can help us calculate the size
		 ; of the gdt.

gdt_descriptor:
	dw gdt_end - gdt_start - 1 ; Size of the GDT, we always write one less
							   ; of the true size.
	dd gdt_start   ; Start of the gdt

;This are useful constants for knowing where our code and data segment descriptors are.
;This is what the segment register needs to contain in protected mode to actually acess
;a segment.
CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start
