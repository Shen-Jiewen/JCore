use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}


// 功能:将内存中缓冲区中的数据写入文件
// 参数:'fd'表示待写入文件的文件描述符
//      'buf'表示内存中缓冲区的起始地址
//      'len'表示内存中缓冲区的长度
// 返回值:返回成功写入的长度
// syscall ID: 64
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

// 功能:退出应用程序,并且将返回值告知批处理系统
// 参数:'exit_code'表示应用程序的返回值
// 返回值:该系统调用不应该返回
// syscall ID: 93
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

// 功能: 主动放弃 CPU 时间片，让出执行权
// 参数: 无参数
// 返回值: 返回 0 表示成功
// syscall ID: 124
pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

// 功能: 获取当前的时间,保存在TimeVal结构体ts中,_tz在我们的实现中忽略
// 返回值: 返回是否执行成功,成功则返回0
// syscall ID: 169
pub fn sys_get_time() -> usize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}