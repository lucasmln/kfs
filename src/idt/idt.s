
.section .text
.extern exception_handler

.macro isr_err_stub n
.global isr_stub_\n
.type isr_stub_\n, @function

isr_stub_\n:
    cli
    push $\n
    jmp isr_common_stub
.endm

.macro isr_no_err_stub n
.global isr_stub_\n
.type isr_stub_\n, @function

isr_stub_\n:
    cli
    push $0
    push $\n
    jmp isr_common_stub
.endm

# Define exception handlers
isr_no_err_stub 0
isr_no_err_stub 1
isr_no_err_stub 2
isr_no_err_stub 3
isr_no_err_stub 4
isr_no_err_stub 5
isr_no_err_stub 6
isr_no_err_stub 7
isr_err_stub    8
isr_no_err_stub 9
isr_err_stub    10
isr_err_stub    11
isr_err_stub    12
isr_err_stub    13
isr_err_stub    14
isr_no_err_stub 15
isr_no_err_stub 16
isr_err_stub    17
isr_no_err_stub 18
isr_no_err_stub 19
isr_no_err_stub 20
isr_no_err_stub 21
isr_no_err_stub 22
isr_no_err_stub 23
isr_no_err_stub 24
isr_no_err_stub 25
isr_no_err_stub 26
isr_no_err_stub 27
isr_no_err_stub 28
isr_no_err_stub 29
isr_err_stub    30
isr_no_err_stub 31


isr_common_stub:
    pusha
    push %ds
    push %es
    push %fs
    push %gs
    mov $0x10, %ax   # Load the Kernel Data Segment descriptor!
    mov %ax, %ds
    mov %ax, %ss
    mov $0x0, %ax
    mov %ax, %es
    mov %ax, %fs 
    mov %ax, %gs
    mov %esp, %eax   # Push us the stack
    push %eax
    #mov exception_handler, %eax
    #call %eax       # A special call, preserves the 'eip' register
    #push (esp)
    call exception_handler
    pop %eax
    pop %gs
    pop %fs
    pop %es
    pop %ds
    popa
    add $8, %esp     # Cleans up the pushed error code and pushed ISR number
    iret



.global load_idt

load_idt:
    movl 4(%esp), %eax
    lidt (%eax)
    sti
    ret


.global test_function
test_function:
    mov $0, %ax
    xor %dx, %dx
    mov $0, %cx
    div %cx
    ret



# .global isr_stub_table
# isr_stub_table:
# .set i 0 
# .rep 32 
#     dd isr_stub_\i
#     inc  
# .endr
