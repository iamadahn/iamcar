#![no_std]
#![no_main]

use static_cell::StaticCell;
use defmt::*;
use embassy_executor::Spawner;
use embassy_time::Timer;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_stm32::time::Hertz;
use embassy_stm32::gpio::{Level, Output, OutputType, Speed};
use embassy_stm32::spi;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use {defmt_rtt as _, panic_probe as _};

mod tasks;
use tasks::led::led_controller_task as led_controller;
use tasks::rc::rc_controller_task as rc_controller;
use tasks::motor::motor_controller_task as motor_controller;

use tasks::rc::InputData;

static INPUT_CHANNEL: StaticCell<Channel<NoopRawMutex, InputData, 1>> = StaticCell::new();

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

    // PB13 - SPI2_SCK, PB14 - SPI2_MISO, PB15 - SPI2_MOSI
    info!("Initialising SPI2 for communication with nrf24.");
    let mut nrf_spi_config = spi::Config::default();
    nrf_spi_config.frequency = Hertz(1_000_000);

    let nrf_spi = spi::Spi::new_blocking(
        peripherals.SPI2,
        peripherals.PB13,
        peripherals.PB15,
        peripherals.PB14,
        nrf_spi_config
    );
    let nrf_ce = Output::new(peripherals.PB12, Level::High, Speed::Low);
    let nrf_cns = Output::new(peripherals.PA8, Level::High, Speed::Low);
    
    info!("Initialising motor pins and pwm channels.");
    let tim1_ch2_pin = PwmPin::new_ch2(peripherals.PA9, OutputType::PushPull);
    let tim1_ch3_pin = PwmPin::new_ch3(peripherals.PA10, OutputType::PushPull);
    let motor_pwm = SimplePwm::new(
        peripherals.TIM1,
        None,
        Some(tim1_ch2_pin),
        Some(tim1_ch3_pin),
        None,
        Hertz(10000),
        Default::default()
    );
    let motor_chs = motor_pwm.split();
    let motor_ch2 = motor_chs.ch2;
    let motor_ch3 = motor_chs.ch3;

    let motor_lf = Output::new(peripherals.PA11, Level::Low, Speed::Low);
    let motor_lb = Output::new(peripherals.PA12, Level::Low, Speed::Low);
    let motor_rf = Output::new(peripherals.PA13, Level::Low, Speed::Low);
    let motor_rb = Output::new(peripherals.PA14, Level::Low, Speed::Low);

    info!("Initialising channels for tasks communication.");
    let input_ch: &'static mut _ = INPUT_CHANNEL.init(Channel::new());
    let input_pub = input_ch.sender();
    let input_sub = input_ch.receiver();

    info!("Spawning tasks.");
    spawner.spawn(led_controller(led)).ok();
    spawner.spawn(rc_controller(nrf_spi, nrf_ce, nrf_cns)).ok();
    spawner.spawn(motor_controller(motor_ch2, motor_ch3, motor_lf, motor_lb, motor_rf, motor_rb, input_sub)).ok(); 

    loop {
        /*
        ch2.set_duty_cycle_fully_off();
        Timer::after_millis(300).await;
        ch2.set_duty_cycle_fraction(1, 4);
        Timer::after_millis(300).await;
        ch2.set_duty_cycle_fraction(1, 2);
        Timer::after_millis(300).await;
        ch2.set_duty_cycle(ch2.max_duty_cycle() - 1);
        */
        Timer::after_millis(300).await;
    }
}

