; This will be a boot sector that will initially load some kernel code in C but will later be changed to Rust code...

[org 0x7c00]
KERNEL_OFFSET equ 0x9000	; This is to say we want to load kernel code with this offset. 4 kb
STACK_ADDR equ 0x8ffc

mov [BOOT_DRIVE], dl		; We are storing the boot drive we are using as a global variable.
							; The BIOS loads boot drive number into dl.

mov bp, STACK_ADDR 				; This sets out stack bottom pointer.
mov sp, bp 					; This top pointer starts at the same location as botton (the stack is empty).

call load_kernel			; Routine to load drive sectors corresponding to kernel.

call switch_to_pm			; Routine to switch CPU into proper 32-bit protected mode.

jmp $

%include "utility/print.asm"
%include "utility/print_32.asm"
%include "utility/load_disk.asm"
%include "gdt.asm"
%include "switch_to_pm.asm"

[bits 16]

load_kernel:

	mov bx, MSG_LOAD_KERNEL
	call println

	mov bx, KERNEL_OFFSET 	; This is were we actually tell the routine
							; to load kernel code with this offset. 
	mov dh, 127				; loading 127 sectors to leave space for 
							; the future
							; when kernel is bigger.  
							; This as far as I can take it for now. 

	mov dl, [BOOT_DRIVE]
	call disk_load								

	ret

[bits 32]
BEGIN_PM:
	call KERNEL_OFFSET 

 	jmp $


BOOT_DRIVE: db 0		; This 0 just saves the space for 1 byte.
MSG_LOAD_KERNEL:
	db "Loading kernel in memory", 0
MSG_PM:
	db "Entered 32-bit PM", 0

times 510-($-$$) db 0
dw 0xaa55
