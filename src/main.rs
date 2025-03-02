#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_stm32::time::Hertz;
use embassy_stm32::gpio::{Level, Output, Speed};
use {defmt_rtt as _, panic_probe as _};

mod led;
use self::led::led_controller_task as led_controller;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    info!("Booting iamrc...");

    let mut config = embassy_stm32::Config::default();
    info!("Initialising clocks...");
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            src: PllSource::HSE,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL9,
        });
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
    }

    info!("Initialising hardware...");
    let peripherals = embassy_stm32::init(config);
    let led = Output::new(peripherals.PB2, Level::High, Speed::Low);

    /* There is W25Q ic on my board connected to SPI1, so it needs to be disabled */
    let mut w25q_cs = Output::new(peripherals.PA4, Level::High, Speed::Low);
    w25q_cs.set_high();

    spawner.spawn(led_controller(led)).ok();

    loop {
        info!("Main: delay");
        Timer::after_millis(500).await;
    }
}

