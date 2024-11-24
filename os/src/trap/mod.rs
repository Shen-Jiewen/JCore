// os/src/trap/mod.rs

// 所有陷入都进入汇编文件Trap.S中的__alltraps
// __alltraps负责最低限度的工作,确保从用户态切换到内核态,并且将控制权转移到trap_handler函数
// 根据不同的陷入类型,执行不同的处理逻辑,比如:定时器中断触发任务抢占,系统调用处理在syscall()中

mod context;

use crate::{syscall::syscall, task::*, timer::set_next_trigger};
use core::arch::global_asm;
use riscv::register::{
    scause::Interrupt, mtvec::TrapMode, scause::{self, Exception, Trap}, sie, stval, stvec
};

global_asm!(include_str!("trap.S"));


pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        // 向stvec寄存器写入两个值
        // 陷入函数的函数地址 陷阱向量的工作模式(直接模式)
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

// 使能定时器中断
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause: scause::Scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub use context::TrapContext;
