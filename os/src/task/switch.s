.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)   # 将寄存器s[n]的值保存到a0+(n+2)*8的位置
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)   # 从a1+(n+2)*8的位置加载值到寄存器s[n]
.endm

    .section .text
    .globl __switch
__switch:
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,  # a0
    #     next_task_cx_ptr: *const TaskContext    # a1
    # )
    
    # save kernel stack of current task
    sd sp, 8(a0)

    # save ra & s0~s11 of current execution
    sd ra, 0(a0)           # 保存返回地址ra到a0指向的地址
    .set n, 0              # 设置循环计数变量n为0
    .rept 12               # 重复下面的指令12次
        SAVE_SN %n         # 调用SAVE_SN宏，保存s0~s11
        .set n, n + 1      # n递增
    .endr

    # restore ra & s0~s11 of next execution
    ld ra, 0(a1)           # 从a1指向的地址加载返回地址ra
    .set n, 0              # 重置循环计数变量n为0
    .rept 12               # 重复下面的指令12次
        LOAD_SN %n         # 调用LOAD_SN宏，加载s0~s11
        .set n, n + 1      # n递增
    .endr

    # restore kernel stack of next task
    ld sp, 8(a1)

    ret                    # 返回，此时CPU的执行上下文已切换到下一个任务
