#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

unsafe extern "C" {
    fn LED_Init();
    fn LED_On();
    fn LED_Off();
    fn BTN_Init();
    fn BTN_Read() -> u8;
}

#[entry]
fn main() -> ! {
    unsafe {
        LED_Init();
        BTN_Init();
    }

    loop {
        let state = unsafe { BTN_Read() }; // 0 or 1
        if state != 0 {
            unsafe { LED_On(); }
        } else {
            unsafe { LED_Off(); }
        }
    }
}

fn delay(mut cycles: u32) {
    while cycles > 0 {
        unsafe {
            core::arch::asm!("nop");
        }
        cycles -= 1;
    }
}
