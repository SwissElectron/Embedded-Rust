#![no_std]
#![no_main]

use core::ptr::{write_volatile,read_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");
    clock_config();
    //definitions of the GPIO configuration addressses for port E
    const GPIOE_MODER_ADDR: *mut u32 = 0x4800_1000 as *mut u32;     //Address of PortE MODER Register          //Mask to set pin9 as Output
    const GPIOE_BSRR_ADDR: *mut u32 = 0x4800_1018 as *mut u32;      //Address of PortE BSSR Register (Bit Set/Reset Register, to avoid writing direclty in output register)
    const PIN_OUT_POS: u32 = 11;
    const BSSR_HALF_SIZE: u32 = 16;

    unsafe {
        let moder_value = read_volatile(GPIOE_MODER_ADDR);
        write_volatile(GPIOE_MODER_ADDR, (moder_value & !(0b11 << (2 * PIN_OUT_POS))) | (0b01 << (2 * PIN_OUT_POS)));
    }

    let mut is_on: bool = false;
    unsafe {
    write_volatile(GPIOE_BSRR_ADDR, 1 << 11);
    }
    loop{
        //if is_on {
        //    unsafe {
        //        write_volatile(GPIOE_BSRR_ADDR, GPIO_ACTIVE << PIN_OUT_POS);
        //    }
        //}
        //else {
        //    unsafe {
        //        write_volatile(GPIOE_BSRR_ADDR,GPIO_ACTIVE << (PIN_OUT_POS + BSSR_HALF_SIZE));
        //    }
        //}
        //for _ in 0..72_000_000{
        //    nop();
        //}
        //is_on = !is_on;
    }
}


fn clock_config(){
    // RCC Configuration
    const RCC_CR_ADDR: *mut u32 = 0x4002_1000 as *mut u32;
    const RCC_CFGR_ADDR: *mut u32 = 0x4002_1004 as *mut u32;
    const RCC_AHBENR_ADDR: *mut u32 = 0x4002_0014 as *mut u32;

    unsafe {

        //Set PLL OFF and wait for PLL_RDY flag
        let mut RCC_CR = read_volatile(RCC_CR_ADDR);
        RCC_CR &= 0xFEFF_FFFF as u32;
        write_volatile(RCC_CR_ADDR, RCC_CR);
        RCC_CR = read_volatile(RCC_CR_ADDR);
        while  RCC_CR & 0x0200_0000 !=0 {
            RCC_CR = read_volatile(RCC_CR_ADDR);
        }

        //Set Clock source  to HSI and wait for HSI to be ready
        RCC_CR = read_volatile(RCC_CR_ADDR);
        RCC_CR |= 0x0000_0001 as u32;
        write_volatile(RCC_CR_ADDR,RCC_CR);
        RCC_CR = read_volatile(RCC_CR_ADDR);
        while RCC_CR & 0x0000_0002 == 0 {
            RCC_CR = read_volatile(RCC_CR_ADDR);
        }

        //Configure PLL to use HSI/2 as clock source, and use PLL to get SYSCLK and HCLK to 72 MHz
        let mut RCC_CFGR: u32 = read_volatile(RCC_CFGR_ADDR);
        RCC_CFGR &= 0xFFFE_FFFF as u32;
        RCC_CFGR |= 0x001C_0000 as u32;
        write_volatile(RCC_CFGR_ADDR, RCC_CFGR);

        RCC_CR |= 0x0100_0000 as u32;
        write_volatile(RCC_CR_ADDR, RCC_CR);
        RCC_CR = read_volatile(RCC_CR_ADDR);
        while RCC_CR & 0x0200_0000 == 0{
            RCC_CR = read_volatile(RCC_CR_ADDR);
        }
        RCC_CFGR = read_volatile(RCC_CFGR_ADDR);
        RCC_CFGR |= 0x0000_0002 as u32;
        write_volatile(RCC_CFGR_ADDR, RCC_CFGR);
        RCC_CFGR = read_volatile(RCC_CFGR_ADDR);
        while RCC_CFGR & 0x0000_000A == 0{
            RCC_CFGR = read_volatile(RCC_CFGR_ADDR);
        }

        RCC_CFGR &= 0xFFFF_FF0F as u32; //Set HCLK Prescaler to no division
        write_volatile(RCC_CFGR_ADDR, RCC_CFGR);

        //Enable clock for GPIOE
        let mut RCC_AHBENR = read_volatile(RCC_AHBENR_ADDR);
        RCC_AHBENR |= 0x0020_0000 as u32;
        write_volatile(RCC_AHBENR_ADDR, RCC_AHBENR);

    }
    return;
}
