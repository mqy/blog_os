#![no_std]
#![no_main]

// build for arm: cargo build --target thumbv7em-none-eabihf

// build in Linux/MacOs/Windows (see .cargo/config.toml): cargo build

// emit asm: rustc src/main.rs --emit asm --target thumbv7em-none-eabihf

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
