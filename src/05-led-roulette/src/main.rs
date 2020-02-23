// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// use aux5::{entry, prelude::*, Delay, Leds};

// #[entry]
// fn main() -> ! {
//     let (mut delay, mut leds): (Delay, Leds) = aux5::init();

//     let ms = 50_u8;
//     loop {
//         for curr in 0..8 {
//             let next = (curr + 1) % 8;

//             leds[next].on();
//             delay.delay_ms(ms);
//             leds[curr].off();
//             delay.delay_ms(ms);
//         }
//     }
// }

//========= using input ===================== 
#![no_main]
#![no_std]

use aux5::{entry, prelude::*};

#[entry]
fn main() -> ! {
    let (mut delay,gpioa, gpiob, gpioc, gpiod, gpioe, gpiof, rcc) = aux5::init();

    rcc.ahbenr.modify(|_, w| {
        w.iopaen().set_bit();
        w.iopben().set_bit();
        w.iopcen().set_bit();
        w.iopden().set_bit();
        w.iopeen().set_bit();
        w.iopfen().set_bit()

    });
    gpiob.moder.modify(|_,w| w.moder0().output());
    gpiob.odr.write(|w| {
        w.odr0().set_bit()
    });
    //=== Port C=====
    gpioc.moder.modify(|_,w| w.moder0().output());
    gpioc.odr.write(|w| {
        w.odr0().set_bit()
    });
    //=== Port D=====
    gpiod.moder.modify(|_,w| w.moder0().output());
    gpiod.odr.write(|w| {
        w.odr0().set_bit()
    });
    //==== Port E ====
    gpioe.moder.modify(|_,w| w.moder9().output());
    gpioe.odr.write(|w| {
        w.odr9().set_bit()
    });
    gpioe.moder.modify(|_,w| w.moder11().output());
    gpioe.odr.write(|w| {
        w.odr11().set_bit()
    });
    //=== Port F=====
    gpiof.moder.modify(|_,w| w.moder9().output());
    gpiof.odr.write(|w| {
        w.odr9().set_bit()
    });

    loop{
    
        // if  gpioa.idr.read().idr0().bit_is_clear(){
            // let ms = 50_u8;
            //     for curr in 0..8 {
                    
            //         let next = (curr + 1) % 8;
        
            //         leds[next].on();
            //         delay.delay_ms(ms);
            //         leds[curr].off();
            //         delay.delay_ms(ms);
            //     }
            //  }
            //  else 
            //  if
            //  gpioa.idr.read().idr0().bit_is_set(){
                //  for i in 0..8
                //  {
                    //  leds[i].off();
                //  }
            //  }
            
    }
}

// #![no_main]
// #![no_std]

// use aux5::{entry, prelude::*};
// use core::ops;
// use hal::gpio::{Output, PushPull};

// #[entry]
// fn main() -> ! {
//     let (mut delay, gpioa, gpiob, gpioc, gpiod, mut gpioe, gpiof, rcc) = aux5::init();

//     rcc.ahbenr.modify (|_,  w| {
//         w.iopeen().set_bit()
//     });

//     gpioe.moder.modify (|_, w| {
//         w.moder9().output();
//         w.moder12().output();
//         w.moder15().output()
        
//     });
    
    
//     loop {
//         let ms = 1000_u16;
//         for i in 1..8
//         {
//             gpioe.odr.modify (|_, w| {
//             w.odr9().set_bit();
//             w.odr12().set_bit();
//             w.odr15().set_bit()
//         }); 
//         delay.delay_ms(ms);
//         // gpioe.odr.modify (|_, w| {
//         //     w.odr9().clear_bit();
//         //     w.odr12().clear_bit();
//         //     w.odr15().clear_bit()
//         // });

//         gpioe.pupdr.modify (|_, w|{
//                w.pupdr9().bits(0b01) 
            
            
//         });

//         }
        

//         // delay.delay_ms(ms);

//         // // gpioe.brr.write().brr9().set_bit();
//         // gpioe.brr.write(|w|{
//         //     w.brr9.set_bit()
//         // });

//         //     gpiof.odr.write(|w| {
// //         w.odr9().set_bit()
// //     });
        
            
        
//     }

// }