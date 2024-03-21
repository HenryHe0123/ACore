    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call booting
    
# set boot stack
    .section .bss.stack
    .globl boot_stack_bound
boot_stack_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
