use defmt::*;
use embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::simple_pwm::SimplePwmChannel;
use embassy_sync::channel::Receiver;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use crate::data_types::{ServoInput, ServoDir};

#[embassy_executor::task]
pub async fn servo_controller_task(
    mut servo: SimplePwmChannel<'static, TIM2>,
    input_sub: Receiver<'static, NoopRawMutex, ServoInput, 1>) {
    loop {
        let input = input_sub.receive().await;
        info!("received input for servo");
    }
}
