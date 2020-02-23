// //! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

pub use cortex_m_rt::entry;
pub use f3::{
    hal::{delay::Delay, prelude},
    led::Leds,
};

use f3::hal::{prelude::*, stm32f30x};

pub fn init() -> (Delay, Leds) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (delay, leds)
}

//====================== using inpput ============

// #![no_std]

// #[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
// extern crate panic_halt; // panic handler

// pub use cortex_m_rt::entry;
// pub use f3::hal::stm32f30x::{gpioa, gpiob, rcc, gpioc};
// pub use f3::{
//     hal::{delay::Delay, prelude},
//     led::Leds,
// };

// use f3::hal::{prelude::*};
// use f3::hal::stm32f30x::{self, GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF, RCC};

// pub fn init() -> (Delay, 
//                 &'static gpioa::RegisterBlock,
//                 &'static gpiob::RegisterBlock,            
//                 &'static gpioc::RegisterBlock,
//                 &'static gpioc::RegisterBlock,
//                 &'static gpioc::RegisterBlock,
//                 &'static gpioc::RegisterBlock, 
//                 &'static rcc::RegisterBlock) 
// {
//     let cp = cortex_m::Peripherals::take().unwrap();
//     let dp = stm32f30x::Peripherals::take().unwrap();

//     let mut flash = dp.FLASH.constrain();
//     let mut rcc = dp.RCC.constrain();

//     let clocks = rcc.cfgr.freeze(&mut flash.acr);

//     let delay = Delay::new(cp.SYST, clocks);

//     let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

//     unsafe { (delay, &*GPIOA::ptr(), &*GPIOB::ptr(), &*GPIOC::ptr(), &*GPIOD::ptr(), &*GPIOE::ptr(), &*GPIOF::ptr(), &*RCC::ptr()) }
// }