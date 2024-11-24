//! 主模块和入口点
//!
//! 内核的各种功能实现为子模块。其中最重要的是：
//!
//! - [`trap`]: 处理从用户空间切换到内核的所有情况
//! - [`task`]: 任务管理
//! - [`syscall`]: 系统调用的处理与实现
//!
//! 操作系统也从这个模块启动。内核代码从 `entry.asm` 开始执行，
//! 随后调用 [`rust_main()`] 来初始化各种功能模块。（具体细节请参见其源代码。）
//!
//! 然后我们调用 [`task::run_first_task()`]，首次进入用户空间。
#![deny(missing_docs)]
#![deny(dead_code)]
#![no_main]
#![no_std]

use core::arch::global_asm;

#[path ="boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
pub mod config;
mod lang_items;
mod sbi;
mod sync;
mod syscall;
mod trap;
mod task;
mod loader;
mod timer;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// Clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
        .fill(0);
    }
}

/// JCore 操作系统的入口函数
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}
