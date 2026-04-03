#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use hal::delay::Delay;
use hal::usart::UartTx;
use {ch32_hal as hal, panic_halt as _};

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    let config = hal::Config::default();
    let p = hal::init(config);
    // Default baud rate is 115200
    let mut uart = UartTx::new_blocking(p.USART1, p.PD6, Default::default()).unwrap();
    
    // Since the default TX port is assigned to PIN8 that collides against SWIO, We need to remap TX to PIN1.
    // See Reference Manual 7.3.2.1 Remap Register 1 (AFIO_PCFR1) for details.
    // https://ch32-riscv-ug.github.io/CH32V003/datasheet_en/CH32V003RM.PDF
    // Caution! The CH32V00XRM(V1.4) is wrong. You need to consult CH32V003's manual.
    let afio = &hal::pac::AFIO;
    afio.pcfr1().modify(|w| {
        // bit 2 (USART1_RM)
        w.set_usart1_rm(true);
        // bit 21 (USART1_RM1)
        w.set_usart1_rm1(false);
    });

    hal::println!("dev init ok");
    let mut delay = Delay;
    loop {
        uart.blocking_write(b"Hello, world!\r\n").unwrap();
        delay.delay_ms(1000);
    }
}
