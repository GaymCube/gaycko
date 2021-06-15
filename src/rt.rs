#![allow(clippy::empty_loop)]

use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr;

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> () {
            // type check the given path
            let f: fn() -> () = $path;

            f()
        }
    };
}

#[panic_handler]
fn panic(panic_info: &PanicInfo<'_>) -> ! {
    writeln!(&mut UART, "PANIC: {}\r", panic_info).ok();

    loop {}
}

extern "C" {
    static _stack_bottom: u8;
    static _stack_top: u8;
    static mut _bss_bottom: u8;
    static mut _bss_top: u8;
}

#[link_section = ".text.crt0"]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        "
        b startup
        .long 0xDEADBEEF
        ",
        options(noreturn),
    )
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn startup() -> ! {
    asm!(
        "
        // First clear all general purpose registers
        li 0, 0
        li 1, 0
        li 2, 0
        li 3, 0
        li 4, 0
        li 5, 0
        li 6, 0
        li 7, 0
        li 8, 0
        li 9, 0
        li 10, 0
        li 11, 0
        li 12, 0
        li 13, 0
        li 14, 0
        li 15, 0
        li 16, 0
        li 16, 0
        li 17, 0
        li 18, 0
        li 19, 0
        li 20, 0
        li 21, 0
        li 22, 0
        li 23, 0
        li 24, 0
        li 25, 0
        li 26, 0
        li 27, 0
        li 28, 0
        li 29, 0
        li 30, 0
        li 31, 0

        // TODO: Ensure the MMU lookup tables are cleared to avoid any weird behaviours on hw.

        // Setup a custom stack just in case (Rust can get very hungry of it sometime so we don't trust the initial stack given)
        lis 1, _stack_top@h
        ori 1, 1, _stack_top@l
        b _start_with_stack
        ",
        options(noreturn),
    )
}

#[no_mangle]
unsafe extern "C" fn clean_bss(start_bss: *mut u8, end_bss: *mut u8) {
    ptr::write_bytes(
        start_bss,
        0,
        end_bss as *const _ as usize - start_bss as *const _ as usize,
    );
}

pub struct UART;

impl Write for UART {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        unsafe {
            puts(s.as_bytes().as_ptr());
        }
        Ok(())
    }
}

// NOTE: This function is hooked by emulator to print logs of homebrews and commercial games.
// BODY: We abuse this fact to have early debugging logs without any framebuffers.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn puts(msg: *const u8) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn _start_with_stack() -> ! {
    // Init stuffs here
    clean_bss(&mut _bss_bottom, &mut _bss_top);

    // Call user entry point
    extern "Rust" {
        fn main() -> ();
    }

    main();

    loop {}
}
