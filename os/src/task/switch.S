.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch
__switch:
    // 将当前栈指针sp的值存储到以a0为基地址的内存地址8(a0)
    sd sp, 8(a0)
    // 将返回地址ra的值存储到以a0为基地址的内存地址0(a0)
    sd ra, 0(a0)
    // 定义一个符号n,并且初始化为0
    .set n, 0
    // 保存第n个s寄存器的值到内存中,偏移量为(n+2)*8字节
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr
    // 从以a1为基地址的内存地址0(a1)加载返回地址ra的值
    ld ra, 0(a1)
    // 重置符号n为0
    .set n, 0
    // 从以a1为基地址的内存中加载s寄存器的值,偏移量为(n+2)*8字节
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    // 从以a1为基地址的内存地址8(a1)加载栈指针sp的值
    ld sp, 8(a1)
    // 返回调用函数
    ret