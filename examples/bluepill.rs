#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

use as5047p::As5047p;
use byteorder::{BigEndian, ByteOrder};
use hal::{delay::Delay, prelude::*, spi::Spi, stm32 as device};
use rt::entry;
use sh::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("init").unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = device::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    // let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let mut cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);

    // SPI1
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    let mut spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        as5047p::MODE,
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut as5047p = As5047p::new(spi, cs).unwrap();

    loop {
        hprintln!("mag: {}", as5047p.get_mag().unwrap());
        hprintln!("angle: {}", as5047p.get_angle().unwrap());
        hprintln!("angle_com: {}", as5047p.get_angle_com().unwrap());
    }
}
