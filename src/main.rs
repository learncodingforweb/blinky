#![no_main]
#![no_std]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;

use defmt_rtt as _;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PD4, Level::High, Speed::VeryHigh);

    loop {
        Timer::after_millis(1000).await;
        led.set_high();
        Timer::after_millis(1000).await;
        led.set_low();
    }
}
