# .altmacro 启用增强宏支持
.altmacro
# 定义宏 SAVE_GP,将寄存器 x\n 的值保存到栈上 n*8(sp) 上
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
# 定义宏 LOAD_GP,从栈上的 n*8(sp) 位置加载值到寄存器 x\n
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

# 段和全局标签定义
    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    # 交换 sp 和 sscratch 的值,现在 sp 指向内核栈, sscratch 指向用户栈
    csrrw sp, sscratch, sp
    # 在内核栈上分配一个 TrapContext,大小为 34*8 字节,用来保存用户态上下文
    addi sp, sp, -34*8
    # 保存通用寄存器
    # 将 x1(ra寄存器) 的值存入 1*8(sp) 的位置
    sd x1, 1*8(sp)
    # 跳过x2(sp)稍后保存
    # 将 x3(gp寄存器) 的值存入 3*8(sp) 的位置
    sd x3, 3*8(sp)
    # 跳过 x4(tp) 程序不使用 tp
    .set n, 5
    # 重复执行 27 次，将 x5-x31 的寄存器按顺序保存在栈上.SAVE_GP %n 使用宏来简化保存操作
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # 保存状态寄存器和用户栈指针
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # 调用陷入处理函数
    mv a0, sp
    call trap_handler

__restore:
    # 从栈中恢复 sstatus 和 sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    # 将 t0 和 t1 的值写回 sstatus 和 sepc,以恢复之前的特权模式和程序计数器
    csrw sstatus, t0
    csrw sepc, t1
    # 从栈中恢复 ssratch，即用户栈指针
    csrw sscratch, t2
    # 恢复通用寄存器
    # 从栈中恢复寄存器 x1(ra)和 x3(gp)
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    # 使用宏 LOAD_GP 恢复从 x5 到 x31 的寄存器
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # 释放 TrapContext,并且返回用户态
    # 释放 TrapContext 所占用的栈空间,将 sp 向上移动 34*8 字节
    addi sp, sp, 34*8
    # 交换 sp 和 sscratch,将 sp 切换回用户栈,sscratch 切换回内核栈
    csrrw sp, sscratch, sp
    # 执行 sret 指令,从内核态返回到用户态,恢复用户程序的执行状态
    sret
