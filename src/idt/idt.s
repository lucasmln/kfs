
.text
.extern exception_handler
.extern irq_handler


# Macro to define isr with error code
.macro isr_err_stub n
.global isr_stub_\n
# .type isr_stub_\n, @function

isr_stub_\n:
    cli
    push $\n
    jmp isr_common_stub
.endm


# Macro to define isr without error code
.macro isr_no_err_stub n
.global isr_stub_\n
# .type isr_stub_\n, @function

isr_stub_\n:
    cli
    push $0 # Pushing a dummy error code
    push $\n
    jmp isr_common_stub
.endm


.macro IRQ  n
.global irq_\n
# .type irq_\n, @function

irq_\n:
    cli
    push $0
    push $\n
    jmp irq_common_stub

.endm


.macro SAVE_REGS
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
    #mov %esp, %eax   # Push us the stack
    #push %eax
.endm

.macro RESTORE_REGS
    #pop %eax
    pop %gs
    pop %fs
    pop %es
    pop %ds
    popa
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


# Define IRQ
IRQ 0
IRQ 1
IRQ 2
IRQ 3
IRQ 4
IRQ 5
IRQ 6
IRQ 7
IRQ 8
IRQ 9
IRQ 10
IRQ 11
IRQ 12
IRQ 13
IRQ 14
IRQ 15

irq_common_stub:
SAVE_REGS
    mov $irq_handler, %eax
    call *%eax
RESTORE_REGS
    add $8, %esp     # Cleans up the pushed error code and pushed ISR number
    iret

isr_common_stub:
SAVE_REGS
    mov $exception_handler, %eax
    call *%eax
RESTORE_REGS
    add $8, %esp     # Cleans up the pushed error code and pushed ISR number
    iret

.global load_idt

load_idt:
    movl 4(%esp), %eax
    lidt (%eax)
    sti
    ret
