.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    # place on trampoline page
    .section .text.trampoline
    .globl __save_trap_ctx
    .globl __restore_ctx
    .align 2
__save_trap_ctx:
    # swap sp, sscratch
    # sscratch is set to where trap_ctx will be saved in user space 
    # (actually on the second highest page, just below the trampoline page)
    csrrw sp, sscratch, sp
    # now sp -> trap_ctx, sscratch -> user stack
    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they have been saved
    csrr t0, sstatus
    csrr t1, sepc
    # save sstatus, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it in trap_ctx
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # all context saved
    # -------------------
    # load kernel_satp into t0
    ld t0, 34*8(sp)
    # load trap_handler into t1
    ld t1, 36*8(sp)
    # set sp to kernel_sp
    ld sp, 35*8(sp)
    # switch to kernel space
    sfence.vma
    csrw satp, t0
    sfence.vma
    # jump to trap_handler (dynamic jump)
    jr t1
    # why can't we use "call trap_handler"?
    # because this pseudo-inst will be replaced by absolute jump by linker & assembler
    # which does not work in this case

    # no more input argument needs to be set
    # trap_ctx is always saved at same place in user space

# execute after trap_handler returns
# a0: *trap_ctx in user space (constant)
# a1: user space token
__restore_ctx:
    # switch to user space
    sfence.vma
    csrw satp, a1
    sfence.vma
    # recover constant *trap_ctx for sscratch
    csrw sscratch, a0 
    mv sp, a0
    # now sp -> trap_ctx in user space
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general-purpuse registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # restore sp (back to user stack)
    ld sp, 2*8(sp) 
    sret
