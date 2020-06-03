#![no_std]
#![no_main]

use panic_semihosting;
use cortex_m_semihosting::hprintln;

use cortex_m_rt as rt;
use rt::{entry, ExceptionFrame};

use atsame7xx_hal as p_hal;
use p_hal::pac as pac;
use core::borrow::Borrow;
// use cortex_m::asm::bkpt;

fn setup_device_peripherals() -> pac::Peripherals {
    let dp = pac::Peripherals::take().unwrap();
    // let cp = cortex_m::Peripherals::take().unwrap();

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



    //disable WDT for now

    dp.WDT.wdt_cr.write(|w| unsafe {
        w.bits(0)
    });


    // clock for PORTC:
    // peripheral id: ID_PIOC         ( 12) /**< \brief Parallel Input/Output Controller (PIOC) */
    //		if (!hri_pmc_get_PCSR0_reg(PMC, (1 << periph_id))) {
    // 			hri_pmc_set_PCSR0_reg(PMC, (1 << periph_id));
    // 		}
    // GPIO for LED0: C8  (PORTC)

    dp
}

//enum gpio_port { GPIO_PORTA, GPIO_PORTB, GPIO_PORTC, GPIO_PORTD, GPIO_PORTE };
//#define LED0 GPIO(GPIO_PORTC, 8)
//#define GPIO(port, pin) ((((port)&0x7u) << 5) + ((pin)&0x1Fu))
//	_gpio_toggle_level((enum gpio_port)GPIO_PORT(pin), 1U << GPIO_PIN(pin));
#[entry]
fn main() -> ! {
    hprintln!("--- MAIN ---").unwrap();
    let dp = setup_device_peripherals();
    hprintln!("--- peripherals ---").unwrap();

    const GPIO_PINMASK: u32 = 0x1F;
    // const GPIO_PORTC: u8 = 2;
    const GPIOC_C8: u32 = 8 & GPIO_PINMASK;

    loop {
        //TODO disable watchdog OR better yet, frequently reset it
        //bkpt();v

        dp.PIOC.pio_odsr.modify(|r,w| unsafe {
            let old_pin_val = r.bits() & GPIOC_C8;
            let next_reg_val = r.bits() & (!old_pin_val);
            w.bits(next_reg_val)
        });

        //	bits_clear = hri_pio_get_ODSR_reg(hw, mask);
        // 	bits_set   = (~bits_clear) & mask;
        // 	hri_pio_set_ODSR_reg(hw, bits_set);
        // 	hri_pio_clear_ODSR_reg(hw, bits_clear);

        cortex_m::asm::delay(1000);
        // dp.WDT.wdt_sr.reset();
        // // dp.WDT.wdt_sr.write(|w| unsafe {
        // //     w.bits(0)
        // // });
    }
}
