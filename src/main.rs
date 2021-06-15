#![no_std]
#![no_main]
#![feature(asm, global_asm, naked_functions, alloc_error_handler)]
#![allow(dead_code)]

use crate::rt::UART;
use core::fmt::Write;

mod rt;

entry!(main);

fn main() {
    writeln!(&mut UART, "HELLO WORLD\n\0").ok();
}
