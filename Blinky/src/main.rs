#![no_std]
#![no_main]
extern crate alloc;

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop{}
}
