//----------------  Template for Nucleo-G474RE  ----------------\\

// This is a basic template to get started with the Nucleo-G474RE board. 
// It initializes the necessary peripherals and enters an infinite loop where it sets an LED pin high and then low. You can modify this template.

// If you want to start a new project, just copy this whole template.


#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

use stm32g4xx_hal as hal;

use crate::hal::prelude::*;
use cortex_m_rt::entry;

#[entry]
fn main() -> !{
    let p = hal::stm32g4::stm32g474::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let gpioa = p.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();


    loop {
        for _ in 0..7000 {
            _ = led.set_high();
        }
        for _ in 0..7000 {
            _ = led.set_low();
        }
    }
}
