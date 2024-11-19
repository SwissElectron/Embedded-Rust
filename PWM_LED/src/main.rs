#![no_std]
#![no_main]


use cortex_m::asm::nop;
use cortex_m_rt::entry;
use stm32f3::stm32f303;
use panic_halt as _;

#[entry]
fn main() -> !{
    
    let peripherals = stm32f303::Peripherals::take().unwrap();

    //Set RCC to send clock to HCLK and to 
    let rcc = &peripherals.RCC;

    rcc.apb2enr.modify(|_,w| w.tim1en().set_bit());
    rcc.ahbenr.modify(|_,w| w.iopeen().set_bit());

    //Set GPIOE Pin 9 as Output , Push-Pull , with Alternate function2 (TIM1_CHAN1)
    let gpioe = &peripherals.GPIOE;
    gpioe.moder.modify(|_,w| w.moder9().alternate());
    gpioe.otyper.modify(|_, w| w.ot9().clear_bit());
    gpioe.ospeedr.modify(|_, w| w.ospeedr9().high_speed());
    gpioe.afrh.modify(|_,w|w.afrh9().af2());

    let tim1 = &peripherals.TIM1;
    tim1.psc.write(|w| w.psc().bits(71));
    tim1.arr.write(|w| w.arr().bits(999));
    tim1.ccr1().write(|w| w.ccr().bits(499));
    tim1.ccmr1_output().modify(|_, w| w.oc1m().pwm_mode1().oc1pe().set_bit());
    tim1.ccer.modify(|_, w| w.cc1e().set_bit());
    tim1.bdtr.modify(|_, w| w.moe().set_bit());

    tim1.cr1.modify(|_, w| w.cen().set_bit());

    let mut duty_cycle: u16 = 10;

    loop {
        // Increment duty cycle by 5% and wrap around at 100%
        duty_cycle = duty_cycle + 5;

        if duty_cycle > 80{
            duty_cycle = 10;
        }
        
        // Calculate CCR1 value based on ARR (999) and duty cycle percentage (0-100%) and keeping value under overflow error
        let new_ccr1_value = ((999u32 * duty_cycle as u32) / 100) as u16;

        if new_ccr1_value <= 999 { // Ensure value does not exceed ARR value
            tim1.ccr1().write(|w| w.ccr().bits(new_ccr1_value as u16));
        }

        // Delay loop to observe changes in PWM
        for _ in 0..72_00 {
            nop();
        }
    }
}
