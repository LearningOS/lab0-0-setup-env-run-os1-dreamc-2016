/*！

本模块实现了 print 和 println 宏。

*/

use crate::sbi::console_putchar;    // 引入console_putchar函数
use core::fmt::{self, Write};   // 引入fmt和Write

struct Stdout;
//  实现Write trait
// 重写write_str方法，将字符串中的每个字符通过console_putchar函数打印到控制台
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}
//  定义print函数
// 通过write_fmt方法将格式化字符串和参数转换为一个std::fmt::Arguments对象，然后调用Stdout的write_fmt方法调用write_str方法将字符串打印到控制台
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export] // 使得宏可以在其他模块中使用

macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}
//  定义println宏
/*
    macro_rules! println：这是一个宏规则定义，它定义了名为 println 的宏。
    ($fmt:literal $(, $($arg:tt)+)?) => { ... }：这是宏规则的模式部分。它匹配传入的参数并执行相应的代码块。
        $fmt:literal：这是一个模式变量，用于匹配并捕获传入的字符串字面量，用于格式化打印。
        $(, $($arg:tt)+)?：这是一个可选的重复模式，用于匹配并捕获零个或多个参数，用于传递给格式化字符串中的占位符。
    $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));：这是宏规则的替换部分。当宏匹配成功时，将执行这段代码。
        $crate：这是一个特殊的变量，用于指代当前宏被展开的 crate。
        console::print：这是一个假设存在的函数调用，用于实际的打印输出。
        format_args!：这是一个宏，用于将格式化字符串和参数转换为一个 std::fmt::Arguments 对象。
        concat!($fmt, "\n")：这是一个宏，用于将 $fmt 和换行符 \n 连接起来，生成一个包含换行符的格式化字符串。
        $(, $($arg)+)?：这是一个可选的重复模式，用于将捕获的参数展开并传递给 format_args! 宏。
 */
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
