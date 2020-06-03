#![no_std]
#![no_main]

use panic_semihosting;
use cortex_m_semihosting::hprintln;

use cortex_m_rt as rt;
use rt::{entry, ExceptionFrame};

use atsame7xx_hal as p_hal;
use p_hal::pac as pac;
use cortex_m::asm::bkpt;

fn setup_peripherals() {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // configuration comes from generated HPL_PMC_CONFIG.H file for SAME70 XPLD sample
    //	_pmc_init_sources();
    // 	_pmc_init_master_clock();
    // 	_pmc_init_program_clock();
    // 	_pmc_init_fs_clock();


    // TODO most of this clock configuration code should move into the HAL
    const CONF_XOSC20M_STARTUP_TIME: u8 = 62;

    // enable XOSC20M external oscillator (3-20 MHz)
    //	data = hri_pmc_read_CKGR_MOR_reg(PMC) & ~CKGR_MOR_MOSCXTBY;
    // 	data |= CKGR_MOR_KEY_PASSWD | CKGR_MOR_MOSCXTEN | CKGR_MOR_MOSCXTST(CONF_XOSC20M_STARTUP_TIME);
    // turn off MOSCXTBY external osc bypass
    dp.PMC.ckgr_mor.modify(|_, w| unsafe {
        w
            //disable crystal oscillator bypass
            .moscxtby().clear_bit()
            // Enable the Main crystal oscillator by setting CKGR_MOR.MOSCXTEN.
            .moscxten().set_bit()
            // Configure the CKGR_MOR.MOSCXTST field with the Main crystal oscillator startup time
            .moscxtst().bits(CONF_XOSC20M_STARTUP_TIME)
            // provide password for writing to this critical config register
            .key().passwd()
    });

    // Wait for PMC_SR.MOSCXTS flag to rise, indicating the
    // end of a startup period of the Main crystal oscillator.
    while dp.PMC.pmc_sr.read().moscxts().bit_is_clear() {
        // spin until crystal oscillator starts
        // TODO timeout and retry?
    }

    dp.PMC.ckgr_mor.modify(|_, w|  {
        w
            // select crystal oscillator as main oscillator
            .moscsel().set_bit()
            // provide password for writing to this critical config register
            .key().passwd()
    });


    //TODO use PMC to setup clocks
    const ID_PIOC: u8 = pac::Interrupt::PIOC as u8;

    // enable peripheral clock for PIOC (PCER0)
    //this is how you set PMC_PCSR0, using PMC_PCER0 :
    dp.PMC.pmc_pcer0.write(|w| unsafe {
        w.bits( (1 << ID_PIOC) as u32 )
    });


    // clock for PORTC:
    // peripheral id: ID_PIOC         ( 12) /**< \brief Parallel Input/Output Controller (PIOC) */
    //		if (!hri_pmc_get_PCSR0_reg(PMC, (1 << periph_id))) {
    // 			hri_pmc_set_PCSR0_reg(PMC, (1 << periph_id));
    // 		}
    // GPIO for LED0: C8  (PORTC)

}

#[entry]
fn main() -> ! {
    hprintln!("--- MAIN ---").unwrap();

    setup_peripherals();

    loop {
        bkpt();
    }
}
