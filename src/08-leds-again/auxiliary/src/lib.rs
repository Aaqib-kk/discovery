//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::asm::bkpt;
pub use cortex_m_rt::entry;
pub use f3::hal::stm32f30x::{gpioc, rcc, gpiob, gpioa};

use f3::hal::stm32f30x::{self, RCC, GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF};

pub fn init() -> ( 
                    &'static gpioa::RegisterBlock, 
                    &'static gpiob::RegisterBlock, 
                    &'static gpioc::RegisterBlock,
                    &'static gpioc::RegisterBlock, 
                    &'static gpioc::RegisterBlock,
                    &'static gpioc::RegisterBlock, 
                    &'static rcc::RegisterBlock) {
    // restrict access to the other peripherals
    (stm32f30x::Peripherals::take().unwrap());

    unsafe { (&*GPIOA::ptr(),  &*GPIOB::ptr(),  &*GPIOC::ptr(), &*GPIOD::ptr(), &*GPIOE::ptr(), &*GPIOF::ptr(), &*RCC::ptr()) }

}

//=====================PWM==================
// #![no_std]

// #[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964 
// extern crate panic_itm; // panic handler

// pub use cortex_m::asm::bkpt;
// pub use cortex_m_rt::entry;
// pub use f3::hal::stm32f30x::{gpioc, rcc, tim1};
// pub use f3::hal::timer::Timer;

// use f3::hal::stm32f30x::{self, GPIOE, RCC, TIM1};

// pub fn init() -> (&'static gpioc::RegisterBlock, &'static rcc::RegisterBlock, &'static tim1::RegisterBlock)     {
//      // restrict access to the other peripherals
//     (stm32f30x::Peripherals::take().unwrap());
    

//     unsafe { (&*GPIOE::ptr(), &*RCC::ptr(), &*TIM1::ptr()) }
// }