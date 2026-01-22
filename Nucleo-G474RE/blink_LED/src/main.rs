//----------------  Template for Nucleo-G474RE  ----------------\\

// This is a basic template to get started with the Nucleo-G474RE board. 
// It initializes the necessary peripherals and enters an infinite loop where it sets an LED pin high. You can modify this template.


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

    // Onboard LED is connected to PA5
    //let led = gpioa.pa5.into_push_pull_output();

    // Offboard LED is connected on a Breadboard to A2 (PA4)
    let mut offboard_led = gpioa.pa4.into_push_pull_output();

    // Button is connected on a Breadboard to D2 (PA10)
    let button = gpioa.pa10.into_pull_up_input();

    loop {
        while button.is_low().unwrap() {
            for _ in 0..10_000 {
                _ = offboard_led.set_high();    
            }
            for _ in 0..10_000 {
                _ = offboard_led.set_low();    
            }
        }            
        _ = offboard_led.set_low();
    }
}
