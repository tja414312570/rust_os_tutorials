// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022 Andre Richter <andre.o.richter@gmail.com>

//! Test if backtracing code detects an invalid frame pointer.

#![feature(format_args_nl)]
#![no_main]
#![no_std]

/// Console tests should time out on the I/O harness in case of panic.
mod panic_wait_forever;

use libkernel::{bsp, cpu, driver, exception, memory};

#[inline(never)]
fn nested() {
    panic!()
}

#[no_mangle]
unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;

    exception::handling_init();
    memory::init();
    bsp::driver::driver_manager().qemu_bring_up_console();

    nested();

    // The QEMU process running this test will be closed by the I/O test harness.
    cpu::wait_forever()
}
