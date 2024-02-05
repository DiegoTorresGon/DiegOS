; load dh sectors, starting from sector 02, to es:bx from drive dl
; cylinder 0, head 0
disk_load:
	pusha
	push dx
					; CHS = 6 / 8 / 21
					; Sector = (LBA mod SectorsPerTrack)+1
					; Cylinder = (LBA/SectorsPerTrack)/NumHeads
					; Head = (LBA/SectorsPerTrack) mod NumHeads
	mov dh, 127

	mov ah, 0x02  	; BIOS specification for reading a sector function
	mov al, dh 		; We want to read dh sectors
	
	mov ch, 0x00	; Select cylinder 0
	mov dh, 0x00	; Select head 0
	mov cl, 0x02	; Start reading from second sector (after boot sector)

	int 0x13 		; BIOS interrupt for loading disk

	pop dx
	jc disk_error_fail
	cmp dh, al
	jne disk_error_sector

	; LBA = 128, cylinder = 0, head = 6 

	
	; mov dh, 41 
	; mov bx, KERNEL_OFFSET
	; add bx, 128 * 512 ; this would be invalid because bx overflows. I need a different way of loading.
	;  
	; mov ah, 0x02  	; BIOS specification for reading a sector function
	; mov al, dh 		; We want to read dh sectors
	; mov ch, 0x00		; Select cylinder 0
	; mov dh, 0x06		; Select head 0
	; mov cl, 0x03		; Start reading from second sector (after boot sector)
	; 
	; int 0x13 			; BIOS interrupt for loading disk
	
	;  pop dx
	; jc disk_error_fail
	; cmp dh, al
	; jne disk_error_sector

	popa
	ret

disk_error_fail:
	mov bx, DISK_ERROR_MSG
	call println
	
	jmp $

disk_error_sector:
	mov bx, DISK_ERROR_SEC
	call println
	
	jmp $

DISK_ERROR_MSG:
	db "E reading", 0

DISK_ERROR_SEC:
	db "Read incomp", 0
