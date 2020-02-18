// //  origial lib

// #![no_std]

// #[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
// extern crate panic_itm; // panic handler
//                         // extern crate panic_semihosting;

// pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
// pub use cortex_m_rt::entry;
// pub use f3::{
//     hal::{delay::Delay, prelude, spi::Spi, time::MonoTimer},
//     l3gd20,
//     l3gd20::I16x3 as OtherI16x3,
//     lsm303dlhc::{I16x3, Sensitivity},
//     L3gd20, Lsm303dlhc,
// };

// pub use f3::hal::stm32f30x::{gpioc, rcc};

// use f3::hal::{
//     i2c::I2c,
//     prelude::*,
//     stm32f30x::{self, GPIOE, RCC},
// };

// pub fn init() -> (L3gd20, Lsm303dlhc, Delay, MonoTimer, ITM) {
//     let cp = cortex_m::Peripherals::take().unwrap();
//     let dp = stm32f30x::Peripherals::take().unwrap();

//     let mut flash = dp.FLASH.constrain();
//     let mut rcc = dp.RCC.constrain();

//     let clocks = rcc.cfgr.freeze(&mut flash.acr);

//     let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
//     let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
//     let mut nss = gpioe
//         .pe3
//         .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
//     nss.set_high();

//     let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
//     let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
//     let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

//     let spi = Spi::spi1(
//         dp.SPI1,
//         (sck, miso, mosi),
//         l3gd20::MODE,
//         1.mhz(),
//         clocks,
//         &mut rcc.apb2,
//     );
//     let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
//     let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//     let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

//     let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

//     let mut l3gd20 = L3gd20::new(spi, nss).unwrap();
//     let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

//     assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

//     let delay = Delay::new(cp.SYST, clocks);
//     let mono_timer = MonoTimer::new(cp.DWT, clocks);

//     (l3gd20, lsm303dlhc, delay, mono_timer, cp.ITM)
// }

//=============================== Version 2 =======================

// #![no_std]

// #[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
// extern crate panic_itm; // panic handler

// pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
// pub use cortex_m_rt::entry;
// pub use f3::{
//     hal::{delay::Delay, prelude, time::MonoTimer},
//     lsm303dlhc::{I16x3, Sensitivity},
//     Lsm303dlhc,
// };

// use f3::hal::{i2c::I2c, prelude::*, stm32f30x};

// pub fn init() -> (Lsm303dlhc, Delay, MonoTimer, ITM) {
//     let cp = cortex_m::Peripherals::take().unwrap();
//     let dp = stm32f30x::Peripherals::take().unwrap();

//     let mut flash = dp.FLASH.constrain();
//     let mut rcc = dp.RCC.constrain();

//     let clocks = rcc.cfgr.freeze(&mut flash.acr);

//     let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
//     let mut nss = gpioe
//         .pe3
//         .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
//     nss.set_high();

//     let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
//     let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//     let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

//     let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

//     let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

//     let delay = Delay::new(cp.SYST, clocks);
//     let mono_timer = MonoTimer::new(cp.DWT, clocks);

//     (lsm303dlhc, delay, mono_timer, cp.ITM)
// }

// ================= Making for output =================
//  origial lib

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler
                        // extern crate panic_semihosting;

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::{
    hal::{delay::Delay, prelude, spi::Spi, time::MonoTimer},
    l3gd20,
    l3gd20::I16x3 as OtherI16x3,
    lsm303dlhc::{I16x3, Sensitivity},
    L3gd20, Lsm303dlhc,
};

pub use f3::hal::stm32f30x::{gpioc, rcc};

use f3::hal::{
    i2c::I2c,
    prelude::*,
    stm32f30x::{self, GPIOE, RCC},
};

pub fn init() -> (
    &'static gpioc::RegisterBlock,
    &'static rcc::RegisterBlock,
    L3gd20,
    Lsm303dlhc,
    Delay,
    MonoTimer,
    ITM,
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high();
    //===================================
    // let mut ledd9 = gpioe
    //     .pe9
    //     .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    // ledd9.set_high();
    //===================================
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let mut l3gd20 = L3gd20::new(spi, nss).unwrap();
    // let mut green = ledd9;
    let lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();

    assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

    let delay = Delay::new(cp.SYST, clocks);
    let mono_timer = MonoTimer::new(cp.DWT, clocks);

    (
        unsafe { &*GPIOE::ptr() },
        unsafe { &*RCC::ptr() },
        l3gd20,
        lsm303dlhc,
        delay,
        mono_timer,
        cp.ITM,
    )
}
