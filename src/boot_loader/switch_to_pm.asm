[bits 16]
; Routing to swtich CPU from 16-bit Real Mode
; into 32-bit Protected Mode.
switch_to_pm:
	cli		; This instructions turns off interrupts.
			; We are switching them of until we set up
			; the interrupt vector in protected mode.
			; Otherwise, interrupts can cause havoc.
	lgdt [gdt_descriptor]

	mov eax, cr0 	; We are seting the first bit of cr0 to 1,
	or eax, 0x1 	; this does the switch to PM.
	mov cr0, eax

	jmp CODE_SEG:init_pm	;We are making a far jump so the CPU
							;will clear all instructions currently
							;on the pipeline and already fetched or
							;decoded intructions.

[bits 32]
;Initialize registers and stack appropiately.
init_pm:
	mov ax, DATA_SEG
	mov ds, ax
	mov ss, ax
	mov es, ax	
	mov fs, ax
	mov gs, ax

	mov ebp, 0x9000	; Update stack position
	mov esp, ebp

	call BEGIN_PM	; This label is main boot code
					; and it's code that will execute
					; normally in 32-bit PM.
