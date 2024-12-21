#![no_std]
#![no_main]

// pub type Volatile<T> = ::volatile::Volatile<T>; // Good
// pub type Volatile<T> = ::volatile::VolatilePtr<'static, T>; // Bad

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

/// As long as there is a volatile read/write, a bootloop will occur.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let buffer = &mut *(0xb8000 as *mut Buffer);
    buffer[0][0].read(); // Bootloop because of this line
    loop {}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScreenChar {
    ascii_character: u8,
    color_code: u8, // ColorCode was repr(transparent) -> u8
}

type Buffer = [[Volatile<ScreenChar>; 80]; 25];
