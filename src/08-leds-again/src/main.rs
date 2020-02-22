// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// use aux8::entry;

// #[entry]
// fn main() -> ! {
//     let (gpioe, rcc) = aux8::init();

//     // TODO initialize GPIOE

//     // Turn on all the LEDs in the compass
//     gpioe.odr.write(|w| {
//         w.odr8().set_bit();
//         w.odr9().set_bit();
//         w.odr10().set_bit();
//         w.odr11().set_bit();
//         w.odr12().set_bit();
//         w.odr13().set_bit();
//         w.odr14().set_bit();
//         w.odr15().set_bit()
//     });

//     aux8::bkpt();

//     loop {}
// }

// //===================================================
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioa, gpiob, gpioc, gpiod, gpioe, gpiof, rcc) = aux8::init();

    //enable GPIOE peripheral
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());

    
        gpioa.moder.modify(|_,w| w.moder0().input());
        gpioe.moder.modify(|_, w| {
            w.moder8().output();
            w.moder9().output();
            w.moder10().input();
            w.moder11().output();
            w.moder12().output();
            w.moder13().output();
            w.moder14().output();
            w.moder15().output()
        });
    
        loop{
            if  gpioa.idr.read().idr0().bit_is_clear(){
                let ms = 50_u8;

                gpioe.odr.write(|w| {
                    w.odr8().set_bit();
                    delay.delay_ms(ms);
                    w.odr9().set_bit();
                    delay.delay_ms(ms);
                    w.odr10().clear_bit();
                    w.odr11().set_bit();
                    w.odr12().set_bit();
                    w.odr13().set_bit();
                    w.odr14().set_bit();
                    w.odr15().set_bit()
                });
                    
                 }
        }
    // gpioc.moder.modify(|_, w| {
    //     w.moder13().output()
    // });
    // gpioc.odr.write(|w| {
    //     w.odr13().set_bit()
    // });

    // //configure the pins asoutputs
   

    // // TODO initialize GPIOE

    // // Turn on all the LEDs in the compass
    

    aux8::bkpt();

    loop {}
}

//===========================PWM====================

// #![no_main]
// #![no_std]

// use aux8::gpioc::afrl::AFRL2R;
// use aux8::{entry, Timer};

// #[inline(never)]
// fn delay(ms: u16) {
//     for i in 0..ms {}
// }

// #[entry]
// unsafe fn main() -> ! {
//     let (gpioe, rcc, tim1) = aux8::init();
//     /// Init LEDs
//     // Initialize IO pin and set it to alternate mode.
//     rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());
//     // Set alternate function for PE9 to AF2.
//     gpioe.afrh.write(|w| w.afrh9().bits(0b0010));
//     gpioe.pupdr.write(|w| w.pupdr9().bits(0b00));
//     gpioe.otyper.write(|w| w.ot9().set_bit());
//     gpioe.ospeedr.write(|w| w.ospeedr9().bits(0b11));
//     gpioe
//         .moder
//         .write(|w| w.moder9().alternate().moder15().output().moder10().output());

//     /// INIT timer
//     // Initialize tim1
//     rcc.apb2enr.modify(|_, w| w.tim1en().set_bit());

//     // Set prescaler
//     tim1.psc.write(|w| w.psc().bits(5000));

//     // Set frequency
//     tim1.arr.write(|w| unsafe { w.arr().bits(2000 - 1) });

//     // Set duty cycle
//     tim1.ccr1.write(|w| unsafe { w.ccr1().bits(1099 - 1) });

//     // Set clock
//     // tim1.cr1.write(|w| w.ckd().bits(00));

//     /// INIT pwm channel
//     // Enable OCx output, and set polarity.
//     tim1.ccer.write(|w| w.cc1e().set_bit().cc1p().clear_bit());

//     // Initialize all registers
//     tim1.egr.write(|w| w.ug().set_bit());

//     // Set pwm mode to mode 1
//     tim1.ccmr1_output
//         .write(|w| unsafe { w.oc1m().bits(0b0110) }.oc1pe().set_bit());

//     // Enable auto reload
//     tim1.cr1
//         .write(|w| w.dir().clear_bit().arpe().set_bit().cen().clear_bit());

//     // Set counter to 0
//     tim1.cnt.write(|w| w.cnt().bits(0));

//     // Enable counter
//     tim1.cr1.write(|w| w.cen().set_bit());

//     loop {
//         if tim1.cnt.read().bits() < tim1.ccr1.read().bits() {
//             gpioe.odr.write(|w| w.odr15().set_bit());
//         } else {
//             gpioe.odr.write(|w| w.odr15().clear_bit());
//         }
//     }
// }

// //================== PWM Version 2 =========================

// //! Example of using a number of timer channels in PWM mode.
// //! Target board: STM32F3DISCOVERY
// #![no_std]
// #![no_main]

// extern crate panic_semihosting;

// use cortex_m_rt::entry;
// //use cortex_m_semihosting::hprintln;
// use embedded_hal::PwmPin;
// use stm32f3::stm32f303;
// use stm32f3xx_hal::flash::FlashExt;
// use stm32f3xx_hal::gpio::GpioExt;
// use stm32f3xx_hal::pwm::{tim16, tim2, tim3, tim8};
// use stm32f3xx_hal::rcc::RccExt;
// use stm32f3xx_hal::time::U32Ext;

// #[entry]
// fn main() -> ! {
//     // Get our peripherals
//     let dp = stm32f303::Peripherals::take().unwrap();

//     // Configure our clocks
//     let mut flash = dp.FLASH.constrain();
//     let mut rcc = dp.RCC.constrain();
//     let clocks = rcc.cfgr.freeze(&mut flash.acr);

//     // Prep the pins we need in their correct alternate function
//     let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
//     let pa4 = gpioa.pa4.into_af2(&mut gpioa.moder, &mut gpioa.afrl);
//     let pa6 = gpioa.pa6.into_af2(&mut gpioa.moder, &mut gpioa.afrl);
//     let pa7 = gpioa.pa7.into_af2(&mut gpioa.moder, &mut gpioa.afrl);

//     let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
//     let pb0 = gpiob.pb0.into_af2(&mut gpiob.moder, &mut gpiob.afrl);
//     let pb1 = gpiob.pb1.into_af2(&mut gpiob.moder, &mut gpiob.afrl);
//     let pb3 = gpiob.pb3.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//     let pb4 = gpiob.pb4.into_af2(&mut gpiob.moder, &mut gpiob.afrl);
//     let pb5 = gpiob.pb5.into_af2(&mut gpiob.moder, &mut gpiob.afrl);
//     let pb7 = gpiob.pb7.into_af10(&mut gpiob.moder, &mut gpiob.afrl);
//     let pb8 = gpiob.pb8.into_af1(&mut gpiob.moder, &mut gpiob.afrh);
//     let pb10 = gpiob.pb10.into_af1(&mut gpiob.moder, &mut gpiob.afrh);

//     let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
//     let pc10 = gpioc.pc10.into_af4(&mut gpioc.moder, &mut gpioc.afrh);

//     // TIM3
//     //
//     // A four channel general purpose timer that's broadly available
//     let tim3_channels = tim3(
//         dp.TIM3,
//         1280,    // resolution of duty cycle
//         50.hz(), // frequency of period
//         &clocks, // To get the timer's clock speed
//     );

//     // Channels without pins cannot be enabled, so we can't forget to
//     // connect a pin.
//     //
//     // DOES NOT COMPILE
//     // tim3_channels.0.enable();

//     // Each channel can be used with a different duty cycle and have many pins
//     let mut tim3_ch1 = tim3_channels.0.output_to_pa6(pa6).output_to_pb4(pb4);
//     tim3_ch1.set_duty(tim3_ch1.get_max_duty() / 20); // 5% duty cyle
//     tim3_ch1.enable();

//     let mut tim3_ch2 = tim3_channels
//         .1
//         .output_to_pa4(pa4)
//         .output_to_pa7(pa7)
//         .output_to_pb5(pb5);
//     tim3_ch2.set_duty(tim3_ch2.get_max_duty() / 40 * 3); // 7.5% duty cyle
//     tim3_ch2.enable();

//     let mut tim3_ch3 = tim3_channels.2.output_to_pb0(pb0);
//     tim3_ch3.set_duty(tim3_ch3.get_max_duty() / 50 * 3); // 6% duty cyle
//     tim3_ch3.enable();

//     let mut tim3_ch4 = tim3_channels.3.output_to_pb1(pb1).output_to_pb7(pb7);
//     tim3_ch4.set_duty(tim3_ch4.get_max_duty() / 10); // 10% duty cyle
//     tim3_ch4.enable();

//     // We can only add valid pins, so we can't do this:
//     //
//     // DOES NOT COMPILE
//     // tim3_ch1.output_to_pb8(pb8);

//     // The pins that we've used are given away so they can't be
//     // accidentaly modified.  This line would "disconnect" our pin
//     // from the channel.
//     //
//     // DOES NOT COMPILE
//     // pb0.into_af15(&mut gpiob.moder, &mut gpiob.afrl);

//     // TIM2
//     //
//     // A 32-bit timer, so we can set a larger resolution
//     let tim2_channels = tim2(
//         dp.TIM2,
//         160000,  // resolution of duty cycle
//         50.hz(), // frequency of period
//         &clocks, // To get the timer's clock speed
//     );

//     let mut tim2_ch3 = tim2_channels.2.output_to_pb10(pb10);
//     tim2_ch3.set_duty(tim2_ch3.get_max_duty() / 20); // 5% duty cyle
//     tim2_ch3.enable();

//     // TIM16
//     //
//     // A single channel timer, so it doesn't return a tuple.  We can
//     // just use it directly
//     let mut tim16_ch1 = tim16(
//         dp.TIM16,
//         1280,    // resolution of duty cycle
//         50.hz(), // frequency of period
//         &clocks, // To get the timer's clock speed
//     )
//     .output_to_pb8(pb8);
//     tim16_ch1.set_duty(tim16_ch1.get_max_duty() / 20); // 5% duty cyle
//     tim16_ch1.enable();

//     // TIM8
//     //
//     // An advanced timer with complementary outputs, so we can output
//     // to complementary pins (works just like standard pins)
//     let tim8_channels = tim8(
//         dp.TIM8,
//         1280,    // resolution of duty cycle
//         50.hz(), // frequency of period
//         &clocks, // To get the timer's clock speed
//     );

//     let mut tim8_ch1 = tim8_channels.0.output_to_pb3(pb3).output_to_pc10(pc10);
//     tim8_ch1.set_duty(tim8_ch1.get_max_duty() / 10); // 10% duty cyle
//     tim8_ch1.enable();

//     // Once we select PB3, we can only use complementary pins (such as
//     // PC10).  These are pins with alternate functions with an 'N' at
//     // the end of the channel (such as TIM8_CH1N) in the reference
//     // manual.  If we had selected a non-complementary pin first, we
//     // would not be able to use PB3 or PC10 (or PA7 which is aready in
//     // use).
//     //
//     // DOES NOT COMPILE
//     // tim8_ch1.output_to_pc6(gpioc.pc6.into_af4(&mut gpioc.moder, &mut gpioc.afrl));

//     loop {}
// }
