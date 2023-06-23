use crate::sbi::shutdown;
use core::panic::PanicInfo;

// 实现panic_handler 
#[panic_handler]  // 标记使编译器使用我们的panic
fn panic(info: &PanicInfo) -> ! { // 传入一个PanicInfo结构体的引用,返回一个!
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
