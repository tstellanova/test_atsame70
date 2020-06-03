#![no_std]
#![no_main]

use panic_halt::{self};
use cortex_m_rt::{entry, ExceptionFrame};

use atsame7xx_hal as p_hal;
use p_hal::pac as pac;

fn setup_peripherals() {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    //TODO use PMC to setup clocks
    let mut pmc = dp.PMC;

}

#[entry]
fn main() -> ! {
    setup_peripherals();

    loop {

    }
}
