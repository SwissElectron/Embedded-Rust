#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f3::stm32f303;
use panic_halt as _;
use cortex_m_semihosting::hprintln;



#[entry]
fn main() -> ! {

    let mut peripherals = stm32f303::Peripherals::take().unwrap();
    let rcc = &peripherals.RCC;

    //Enable clock for GPIOA and Timer8
    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());
    rcc.apb2enr.modify(|_, w| w.tim8en().set_bit());
    
    let gpioa = &peripherals.GPIOA;
    // Set PA5 as output, push-pull- high-speed, alternate function 4 (as TIM8 CH1)
    gpioa.moder.modify(|_, w| w.moder7().output());
    gpioa.ospeedr.modify(|_, w| w.ospeedr7().high_speed());
    gpioa.otyper.modify(|_, w| w.ot7().push_pull());
    gpioa.afrl.modify(|_,w| w.afrl4().af4());

    let tim8 = &peripherals.TIM8;
    tim8.psc.modify(|_,w| w.psc().bits(71)); //1000 kHz
    tim8.arr.modify(|_,w| w.arr().bits(19999)); //20 ms
    tim8.ccr1().write(|w| w.ccr().bits(1999)); // 1ms pulse 
    tim8.ccmr1_output().modify(|_,w| w.oc1m().pwm_mode1().oc1pe().set_bit());
    tim8.ccer.modify(|_,w| w.cc1e().set_bit());
    tim8.bdtr.modify(|_,w| w.moe().set_bit());
    tim8.cr1.modify(|_,w| w.cen().set_bit());

    loop {
        hprintln!("Looping...");
    }
}
