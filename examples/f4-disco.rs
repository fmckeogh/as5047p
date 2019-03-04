#![no_main]
#![no_std]

extern crate panic_itm;

use as5047p::As5047p;
use byteorder::{BigEndian, ByteOrder};
use cortex_m::iprintln;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f4xx_hal::{delay::Delay, prelude::*, spi::Spi, stm32 as device};

const CPU_HZ: u32 = 64_000_000;

#[entry]
fn main() -> ! {
    let peripherals = cortex_m::Peripherals::take().unwrap();
    let device = device::Peripherals::take().unwrap();

    let clocks = {
        // Power mode
        device.PWR.cr.modify(|_, w| unsafe { w.vos().bits(0x11) });
        // Flash latency
        device
            .FLASH
            .acr
            .modify(|_, w| unsafe { w.latency().bits(0x11) });

        let rcc = device.RCC.constrain();
        rcc.cfgr
            .sysclk(CPU_HZ.hz())
            .pclk1((CPU_HZ / 2).hz())
            .pclk2(CPU_HZ.hz())
            .hclk(CPU_HZ.hz())
            .freeze()
    };

    let mut gpioa = device.GPIOA.split();
    // let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let mut cs = gpioa.pa4.into_push_pull_output();

    // SPI1
    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    let mut spi = Spi::spi1(
        device.SPI1,
        (sck, miso, mosi),
        as5047p::MODE,
        10_000_000.hz(),
        clocks,
    );

    let mut itm = peripherals.ITM;

    let mut delay = Delay::new(peripherals.SYST, clocks);
    //let mut as5047p = As5047p::new(spi, cs).unwrap();
    let mut count = 0u64;

    loop {
        /*
        let mut buf1 = [0xFF, 0xFF];
        cs.set_low();
        spi.transfer(&mut buf1).unwrap();
        cs.set_high();
        */

        let mut buf2 = [0x00, 0x00];
        cs.set_low();
        spi.transfer(&mut buf2).unwrap();
        cs.set_high();

        //buf2[0] & 0x3F;
        iprintln!(&mut itm.stim[0], "out: {:?}", buf2);
        //hprintln!("mag: {}", as5047p.get_mag().unwrap());
        //hprintln!("angle: {}", as5047p.get_angle().unwrap());
        //hprintln!("angle_com: {}", as5047p.get_angle_com().unwrap());
    }
}
