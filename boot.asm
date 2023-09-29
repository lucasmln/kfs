bits 32

section .multiboot               ;according to multiboot spec
        dd 0x1BADB002            ;set magic number for
                                 ;bootloader
        dd 0x0                   ;set flags
        dd - (0x1BADB002 + 0x0)  ;set checksum

section .text
global start
extern main                      ;defined in the C file

read_port:
	mov edx, [esp + 4]
	in al, dx	
	ret

write_port:
	mov   edx, [esp + 4]    
	mov   al, [esp + 4 + 4]  
	out   dx, al  
	ret

keyboard_handler:                 
	call    keyboard_handler_main
	iretd


start:
        cli                      ;block interrupts
        mov esp, stack_space     ;set stack pointer
        call main
        hlt                      ;halt the CPU

section .bss
resb 8192                        ;8KB for stack
stack_space:

