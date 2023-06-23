#![allow(unused)] //一个属性，用于忽略未使用的代码警告

const SBI_SET_TIMER: usize = 0;  // 设置计时器
const SBI_CONSOLE_PUTCHAR: usize = 1;  // 打印字符
const SBI_CONSOLE_GETCHAR: usize = 2;   // 获取字符
const SBI_SHUTDOWN: usize = 8;  // 关闭机器


// 调用SBI
/*
    which: 要调用的SBI函数编号
    arg0: 传递给SBI函数的第一个参数
    arg1: 传递给SBI函数的第二个参数
    arg2: 传递给SBI函数的第三个参数
    return: SBI函数的返回值
    RISC-V通用寄存器 x17 （即存放系统调用号）、 x10 （存放 fd ，也保存返回值） 、 x11 （存放 buf ）和 x12 （存放 len ）保存
 */
#[inline(always)] //内联函数
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}
//  实现打印字符的函数
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}
//  实现获取字符的函数
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}
//  实现设置计时器关闭的函数
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}
