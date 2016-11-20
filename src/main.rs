#![no_main]
#![no_std]

extern crate stm32f411e_disco;

use core::ptr; // For direct memory read/write.
use stm32f411e_disco::peripheral;

#[no_mangle]
pub fn main() -> ! {
    // Only use 1 of the following sections
    // -------------- Using stm32f411e-disco::peripheral -------------
    let rcc = unsafe { peripheral::rcc_mut() };
    // Set the IOPAEN bit of the AHBENR register, which is in the RCC register block, to 1.
    rcc.ahb1enr.modify(|_, w| w.gpioden(true));

    let gpiod = unsafe { peripheral::gpiod_mut() };
    gpiod.moder.modify(|_, w| {
        w.moder12(0b01)
            .moder13(0b01)
            .moder14(0b01)
            .moder15(0b01)
    });

    let bsrr = &peripheral::gpiod().bsrr;
    loop {
        bsrr.write(|w| w.bs15(true));
        for x in 0..30000 {}

        bsrr.write(|w| w.br15(true));
        for x in 0..30000 {}
    }
    // ---------------------------------------------------------------

    // -------------- Using direct memory writes ---------------------
    // const RCC: usize = 0x40023800;
    // const RCC_AHB1ENR: usize = RCC + 0x30;
    // unsafe { ptr::write_volatile(RCC_AHB1ENR as *mut usize, (1 << 3)); }
    //
    // const GPIOD: usize = 0x40020c00;
    // const GPIOD_MODER: usize = GPIOD;
    // unsafe { ptr::write_volatile(GPIOD_MODER as *mut usize, (1 << 30)); } // moder15
    //
    // const GPIOD_BSRR: usize = GPIOD + 0x18;
    //
    // loop {
    //     unsafe { ptr::write_volatile(GPIOD_BSRR as *mut usize, (1 << 15)); }
    //     for x in 0..30000 {}
    //
    //     unsafe { ptr::write_volatile(GPIOD_BSRR as *mut usize, (1 << 31)); }
    //     for x in 0..30000 {}
    // }
    // ---------------------------------------------------------------
}
