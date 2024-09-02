BITS 16
ORG 0X7C00

start:
    mov ax, 0x07C0
    add ax, 288
    mov ss, ax
    mov sp, 4096

    mov ax, 0x0000
    mov ds, ax
    mov es, ax

    call bootloader_main

hang:
    jmp hang

bootloader_main:
    jmp 0x0000:0x7E00

times 510 - ($ - $$) db 0
dw 0xAA55