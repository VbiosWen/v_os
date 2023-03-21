// main.rs
#![no_std] //不链接rust的标准库
#![no_main] // 禁用所有rust层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;
static HELLO: &[u8] = b"Hello World!";

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for ele in tests {
        ele();
    }
    exit_qemu(QemuExitCode::SUCCESS);
}


#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 因为链接器会寻找一个名为"_start"的函数，所以这个函数就是入口点
    // 默认命名为'_start'
    //    vga_buffer::print_something();

    //    use core::fmt::Write;
    //    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    //    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello World!");
    #[cfg(test)]
    test_main();
    panic!("Some panic message");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed\n]");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::FAILED);
    loop {}
}

pub enum QemuExitCode {
    SUCCESS = 0x00,
    FAILED = 0x011,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
