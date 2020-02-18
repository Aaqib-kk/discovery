//// This code is working for angle measurements =============================

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux16::{entry, gpioc, iprint, iprintln, prelude::*, I16x3, OtherI16x3, Sensitivity};
use core::f32::consts::PI;
use m::Float;

#[entry]
fn main() -> ! {
    let (gpioe, rcc, mut l3gd20, mut lsm303dlhc, mut delay, _mono_timer, mut itm) = aux16::init();

    // extend sensing range to `[-12g, +12g]`
    lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();
    loop {

        //=============== Gyroscope=======================
        // let _m = l3gd20.all().unwrap();
        // iprintln!(&mut itm.stim[0], "{:?}", _m);
        let OtherI16x3 { x, y, z } = l3gd20.gyro().unwrap();
        let mut xx = x as f32;

        // iprintln!(&mut itm.stim[0], "{:?}", xx);

        //================= Accelerometer==================
        const SENSITIVITY: f32 = 12. / (1 << 14) as f32;

        let I16x3 { x, y, z } = lsm303dlhc.accel().unwrap();

        let x = f32::from(x) * SENSITIVITY;
        let y = f32::from(y) * SENSITIVITY;
        let z = f32::from(z) * SENSITIVITY;

        // iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));

        //============ Converting to degree of rotation ===========
        let mut fxg = x as f32 * 0.04 as f32;
        let mut fyg = y as f32 * 0.04 as f32;
        let mut fzg = z as f32 * 0.04 as f32;
        // let mut fig = i;

        let xg: f32 = 0.;
        let mut x = fxg * 0.63 as f32 + (xg * (1.0 - 0.63) as f32);
        let mut xg = x;
        let yg: f32 = 0.;
        let mut y = fyg * 0.63 as f32 + (yg * (1.0 - 0.63) as f32);
        let mut yg = y;
        let zg: f32 = 0.;
        let mut z = fzg * 0.63 as f32 + (zg * (1.0 - 0.63) as f32);
        let mut zg = z;

        let x = x as f32;
        let y = y as f32;

        let mut pitch = (((y * y + z * z).sqrt()).atan2(x) * 180.0) / PI;
        let mut pgyro = xx * 0.00005;

        let CFILTIER_COEF = 0.95;
        let mut angle: f32 = -4.7;
        let angle = ((CFILTIER_COEF) * ((angle) + pgyro) + (1.0 - CFILTIER_COEF) * (pitch));
        let mut modified_angle = angle * 20.0;
        if modified_angle > 0. {
            iprintln!(
                &mut itm.stim[0],
                "Turning Angle Clockwise = {:?} ",
                modified_angle
            );
        } else {
            iprintln!(
                &mut itm.stim[0],
                "Turning Angle CounterClockwise = {:?} ",
                modified_angle
            );
        }

        //         //enable GPIOE peripheral
        rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());

        //configure the pins asoutputs
        gpioe.moder.modify(|_, w| {
            w.moder8().output();
            w.moder9().output();
            w.moder10().output();
            w.moder11().output();
            w.moder12().output();
            w.moder13().output();
            w.moder14().output();
            w.moder15().output()
        });

        if modified_angle > 10.0 && modified_angle < 25.0 {
            gpioe.odr.write(|w| {
                w.odr8().set_bit();
                w.odr9().set_bit()
            });
        } else if modified_angle > 25.0 && modified_angle < 40.0 {
            gpioe.odr.write(|w| {
                w.odr8().set_bit();
                w.odr9().set_bit();
                w.odr10().set_bit();
                w.odr11().set_bit()
            });
        } else if modified_angle > 40.0 && modified_angle < 55.0 {
            gpioe.odr.write(|w| {
                w.odr8().set_bit();
                w.odr9().set_bit();
                w.odr10().set_bit();
                w.odr11().set_bit();
                w.odr12().set_bit();
                w.odr13().set_bit()
            });
        } else if modified_angle > 55.0 && modified_angle < 70.0 {
            gpioe.odr.write(|w| {
                w.odr8().set_bit();
                w.odr9().set_bit();
                w.odr10().set_bit();
                w.odr11().set_bit();
                w.odr12().set_bit();
                w.odr13().set_bit();
                w.odr14().set_bit();
                w.odr15().set_bit()
            });
        }
        //=============== reversing =================
        else if modified_angle < -10.0 && modified_angle > -25.0 {
            gpioe.odr.write(|w| {
                w.odr15().set_bit();
                w.odr14().set_bit()
            });
        } else if modified_angle < -25.0 && modified_angle > -40.0 {
            gpioe.odr.write(|w| {
                w.odr15().set_bit();
                w.odr14().set_bit();
                w.odr13().set_bit();
                w.odr12().set_bit()
            });
        } else if modified_angle < -40.0 && modified_angle > -55.0 {
            gpioe.odr.write(|w| {
                w.odr15().set_bit();
                w.odr14().set_bit();
                w.odr13().set_bit();
                w.odr12().set_bit();
                w.odr11().set_bit();
                w.odr10().set_bit()
            });
        } else if modified_angle < -55.0 && modified_angle > -70.0 {
            gpioe.odr.write(|w| {
                w.odr15().set_bit();
                w.odr14().set_bit();
                w.odr13().set_bit();
                w.odr12().set_bit();
                w.odr11().set_bit();
                w.odr10().set_bit();
                w.odr8().set_bit();
                w.odr9().set_bit()
            });
        } else {
            gpioe.odr.write(|w| {
                w.odr8().clear_bit();
                w.odr9().clear_bit();
                w.odr10().clear_bit();
                w.odr11().clear_bit();
                w.odr12().clear_bit();
                w.odr13().clear_bit();
                w.odr14().clear_bit();
                w.odr15().clear_bit()
            });
        }

        delay.delay_ms(4_00_u16);
    }
}

// ============= version 2 ===================

// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux16::{entry, iprint, iprintln, prelude::*, I16x3, Sensitivity};
// use m::Float;

// #[entry]
// fn main() -> ! {
//     const SENSITIVITY: f32 = 12. / (1 << 14) as f32;
//     const THRESHOLD: f32 = 0.5;

//     let (mut lsm303dlhc, mut delay, mono_timer, mut itm) = aux16::init();

//     lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();

//     let measurement_time = mono_timer.frequency().0; // 1 second in ticks
//     let mut instant = None;
//     let mut max_g = 0.;
//     loop {
//         let g_x = f32::from(lsm303dlhc.accel().unwrap().x).abs() * SENSITIVITY;

//         match instant {
//             None => {
//                 // If acceleration goes above a threshold, we start measuring
//                 if g_x > THRESHOLD {
//                     iprintln!(&mut itm.stim[0], "START!");

//                     max_g = g_x;
//                     instant = Some(mono_timer.now());
//                 }
//             }
//             // Still measuring
//             Some(ref instant) if instant.elapsed() < measurement_time => {
//                 if g_x > max_g {
//                     max_g = g_x;
//                 }
//             }
//             _ => {
//                 // Report max value
//                 iprintln!(&mut itm.stim[0], "Max acceleration: {}g", max_g);

//                 // Measurement done
//                 instant = None;

//                 // Reset
//                 max_g = 0.;
//             }
//         }

//         delay.delay_ms(50_u8);
//     }
// // }
// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux16::{entry, iprint, iprintln, prelude::*, I16x3, Sensitivity};
// use m::Float;

// #[entry]
// fn main() -> ! {
//     const SENSITIVITY: f32 = 12. / (1 << 14) as f32;
//     const THRESHOLD: f32 = 0.5;

//     let (mut lsm303dlhc, mut delay, mono_timer, mut itm) = aux16::init();

//     lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();

//     let measurement_time = mono_timer.frequency().0; // 1 second in ticks
//     let mut instant = None;
//     let mut max_g = 0.;
//     loop {
//         let g_x = f32::from(lsm303dlhc.accel().unwrap().x).abs() * SENSITIVITY*10.;

//         match instant {
//             None => {
//                 // If acceleration goes above a threshold, we start measuring
//                 if g_x > THRESHOLD {
//                     iprintln!(&mut itm.stim[0], "START!");

//                     max_g = g_x;
//                     instant = Some(mono_timer.now());
//                 }
//             }
//             // Still measuring
//             Some(ref instant) if instant.elapsed() < measurement_time => {
//                 if g_x > max_g {
//                     max_g = g_x;
//                 }
//             }
//             _ => {
//                 // Report max value
//                 iprintln!(&mut itm.stim[0], "Max acceleration: {}g", max_g);

//                 // Measurement done
//                 instant = None;

//                 // Reset
//                 max_g = 0.;
//             }
//         }

//         delay.delay_ms(50_u8);
//     }
// }

//============ angle measurement origial  ================

// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// #[allow(unused_imports)]
// use aux16::{entry, iprint, iprintln, prelude::*, I16x3, OtherI16x3, Sensitivity};
// use core::f32::consts::PI;
// use m::Float;

// #[entry]
// fn main() -> ! {
//     let (mut l3gd20, mut lsm303dlhc, mut delay, _mono_timer, mut itm) = aux16::init();

//     // extend sensing range to `[-12g, +12g]`
//     lsm303dlhc.set_accel_sensitivity(Sensitivity::G12).unwrap();

//     loop {
//         //=============== Gyroscope=======================

//         // let _m = l3gd20.all().unwrap();
//         // iprintln!(&mut itm.stim[0], "{:?}", _m);
//         let OtherI16x3 { x, y, z } = l3gd20.gyro().unwrap();
//         let mut xx = x as f32;
//         let mut yy = y as f32;

//         // iprintln!(&mut itm.stim[0], "{:?}", xx);

//         //================= Accelerometer==================
//         const SENSITIVITY: f32 = 12. / (1 << 14) as f32;

//         let I16x3 { x, y, z } = lsm303dlhc.accel().unwrap();

//         let x = f32::from(x) * SENSITIVITY;
//         let y = f32::from(y) * SENSITIVITY;
//         let z = f32::from(z) * SENSITIVITY;

//         // iprintln!(&mut itm.stim[0], "{:?}", (x, y, z));

//         //============ Converting to degree of rotation ===========
//         let mut fxg = x as f32 * 0.04 as f32;
//         let mut fyg = y as f32 * 0.04 as f32;
//         let mut fzg = z as f32 * 0.04 as f32;
//         // let mut fig = i;

//         let xg: f32 = 0.;
//         let mut x = fxg * 0.63 as f32 + (xg * (1.0 - 0.63) as f32);
//         let mut xg = x;
//         let yg: f32 = 0.;
//         let mut y = fyg * 0.63 as f32 + (yg * (1.0 - 0.63) as f32);
//         let mut yg = y;
//         let zg: f32 = 0.;
//         let mut z = fzg * 0.63 as f32 + (zg * (1.0 - 0.63) as f32);
//         let mut zg = z;

//         let x = x as f32;
//         let y = y as f32;

//         let mut pitch = (((y * y + z * z).sqrt()).atan2(x) * 180.0) / PI;
//         let mut pgyro = xx * 0.00005;
//         let mut pgyroy = yy * 0.00005;

//         let CFILTIER_COEF = 0.95;
//         let mut angle: f32 = -4.7;
//         let angle = ((CFILTIER_COEF) * ((angle) + pgyro) + (1.0 - CFILTIER_COEF) * (pitch));
//         iprintln!(&mut itm.stim[0], "Angle = {:?}", angle * 20.0);

//         //         //enable GPIOE peripheral
//         // rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());

//         // //     //configure the pins asoutputs
//         // gpioe.moder.modify(|_, w| {
//         //     w.moder8().output();
//         //     w.moder9().output();
//         //     w.moder10().output();
//         //     w.moder11().output();
//         //     w.moder12().output();
//         //     w.moder13().output();
//         //     w.moder14().output();
//         //     w.moder15().output()
//         // });

//         // gpioe.odr.write(|w| match angle {
//         //     1.0 => w.odr1().set_bit(),
//         //     2.0 => w.odr2().set_bit(),
//         //     3.0 => w.odr3().set_bit(),
//         //     4.0 => w.odr4().set_bit(),
//         //     5.0 => w.odr5().set_bit(),
//         //     6.0 => w.odr6().set_bit(),
//         //     7.0 => w.odr6().set_bit(),
//         //     8.0 => w.odr8().set_bit(),
//         //     -1.0 => w.odr1().clear_bit(),
//         //     -2.0 => w.odr2().clear_bit(),
//         //     -3.0 => w.odr3().clear_bit(),
//         //     -4.0 => w.odr4().clear_bit(),
//         //     -5.0 => w.odr5().clear_bit(),
//         //     -6.0 => w.odr6().clear_bit(),
//         //     -7.0 => w.odr6().clear_bit(),
//         //     -8.0 => w.odr8().clear_bit(),
//         //     _ => w.odr8().clear_bit(),
//         // });

//         delay.delay_ms(4_00_u16);
//     }
// }
