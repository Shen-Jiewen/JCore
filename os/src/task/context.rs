// os/src/task/context.rs

// 任务上下文
#[derive(Copy, Clone)]  //允许结构体的浅复制和克隆
#[repr(C)]              //指定内存布局为C语言风格,确保和底层代码(如汇编)兼容
pub struct TaskContext {
    // return address ( e.g. __restore ) of __switch ASM function
    // 返回地址 (例如 __restore), __switch 汇编函数的返回地址
    ra: usize,      // 保存返回地址的寄存器,指向 __restore 汇编函数
    // kernel stack pointer of app
    // 应用的内核栈指针
    sp: usize,      // 栈指针,指向内核栈的位置
    // called saved registers: s 0..11
    // 被调用者保存的寄存器 s0..s11
    s: [usize; 12], // s0到s11寄存器的数组,用于保存调用上下文中的重要数据
}

impl TaskContext {
    // init task context
    // 初始化任务上下文,将所有寄存器设置为0
    pub fn zero_init() -> Self {
        Self {
            ra: 0,          // 将返回地址寄存器初始化为0
            sp: 0,          // 将栈指针初始化为0
            s: [0; 12],     // 将被调用者保存寄存器初始化为0
        }
    }

    // 设置任务上下文,用于在任务切换时跳转到__restore汇编函数
    // 参数: kstack_ptr 内核栈的指针位置
    // set task context {__restore ASM function, kernel stack, s_0..12 }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            // 声明外部汇编函数 __restore
            fn __restore();
        }
        Self {
            ra: __restore as usize,     // 设置返回地址为 __restore 函数的地址
            sp: kstack_ptr,             // 设置内核栈指针
            s: [0; 12],                 // 初始化被调用者保存寄存器为0
        }
    }
}