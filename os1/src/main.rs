#![no_std] // 移除标准库
#![no_main] // 移除main函数
#![feature(panic_info_message)] // 启用 panic_info_message 特性，从而使得我们可以使用 panic::Location 和 panic::PanicInfo::message 这两个特性

use log::*;

#[macro_use] // 使用其他模块中定义的宏
mod console;
mod lang_items;
mod logging;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));    // 引入入口汇编代码

// 清空.bss段
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]  // 保留名称用以和ABI交互
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); 
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();    // 清空.bss段
    logging::init();    // 初始化日志
    println!("Hello, world!");  // 打印Hello, world!
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize); // 打印.text段的地址范围
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize); // 打印.rodata段的地址范围
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);  // 打印.data段的地址范围
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    ); // 打印boot_stack的地址范围
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize); // 打印.bss段的地址范围
    panic!("Shutdown machine!");   // 关闭机器
}
