#![no_main] // 禁用所有 Rust 层级的入口点
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

mod vga_buffer;


#[unsafe(no_mangle)] // 不重整函数名
pub extern "C" fn _start() -> ! {
    println!("Hello World");


    #[cfg(test)]
    test_main();
    panic!("noo");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Failed);
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success=0x10,
    Failed=0x11,
}

pub fn exit_qemu(exit_code:QemuExitCode){
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port=Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


