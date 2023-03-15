// main.rs
#![no_std] //不链接rust的标准库
#![no_main] // 禁用所有rust层级的入口点
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start()-> !{
    // 因为链接器会寻找一个名为"_start"的函数，所以这个函数就是入口点
    // 默认命名为'_start'
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}
