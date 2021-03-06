#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    aux7::init();

    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        const GPIOC_BSRR: u32 = 0x4800_0818;

        // Turn on the "North" LED (red)
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED (green)
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);

         // Turn on the "East" LED (green)
         *(GPIOC_BSRR as *mut u32) = 1 << 12;

         // Turn off the "North" LED
         *(GPIOC_BSRR as *mut u32) = 1 << (12 + 16);

    }

    loop {}
}
