[bits 32]
[extern _start]

call _start
jmp $

;times 10240-($-$$) db 0
