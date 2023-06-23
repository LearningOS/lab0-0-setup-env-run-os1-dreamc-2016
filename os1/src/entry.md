.section .text.entry    ; 将接下来的代码段标记为 .text.entry，表示这是程序的入口代码段。
.globl _start           ; 将 _start 符号声明为全局符号，表示程序的入口点。
_start:                 ; 这是一个标签，表示程序的入口点。
    la sp, boot_stack_top   ; 将 boot_stack_top 符号的地址加载到栈指针寄存器 sp 中，设置栈的初始位置。
    call rust_main          ; 调用 rust_main 函数，进入 Rust 代码的执行。

.section .bss.stack     ; 将接下来的代码段标记为 .bss.stack，表示这是未初始化的数据段。
.globl boot_stack       ; 将 boot_stack 符号声明为全局符号，表示未初始化的堆栈空间的起始位置。
boot_stack:             ; 这是一个标签，表示未初始化的堆栈空间的起始位置。
    .space 4096 * 16    ; 分配 16 个页面大小（4096 字节）的空间作为堆栈空间。
.globl boot_stack_top   ; 将 boot_stack_top 符号声明为全局符号，表示未初始化的堆栈空间的结束位置。
boot_stack_top:         ; 这是一个标签，表示未初始化的堆栈空间的结束位置。