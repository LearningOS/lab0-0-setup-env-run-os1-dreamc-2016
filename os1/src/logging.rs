/*！

本模块利用 log crate 为你提供了日志功能，使用方式见 main.rs.

*/

use log::{self, Level, LevelFilter, Log, Metadata, Record}; // 引入log相关的结构体

struct SimpleLogger;
//  实现Log trait
impl Log for SimpleLogger {
    // 重写enabled方法，使得日志总是可用
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    // 重写log方法，使得日志打印到控制台
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    // 重写flush方法，使得日志总是刷新
    fn flush(&self) {}
}
//  初始化日志
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}
