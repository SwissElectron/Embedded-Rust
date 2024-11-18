#![no_std]
#![no_main]

use cortex_m::peripheral::{Peripherals,syst};
use cortex_m_rt::entry;
use cortex_m::asm::nop;
use cortex_m_semihosting::hprintln;
use stm32f3::stm32f303;

use panic_halt as _;

#[entry]
fn main() -> !{
    let peripherals = stm32f303::Peripherals::take().unwrap();
    let rcc = &peripherals.RCC;
    let gpioe = &peripherals.GPIOE;

    rcc.ahbenr.write(|w| w.iopeen().set_bit());
    gpioe.moder.write(|w| w.moder9().bits(0b01));

    let mut is_on: bool = false;
    hprintln!("Starting Loop....");
    loop{

        if !is_on {
            gpioe.odr.write(|w| w.odr9().set_bit() );
        }
        else {
            gpioe.odr.write(|w| w.odr9().clear_bit() );
        }

        for _ in 0..72000 {
            nop();
        }
        is_on = !is_on;
    }
}
