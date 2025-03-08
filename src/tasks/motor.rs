use embassy_stm32::peripherals::TIM1;
use embassy_stm32::timer::simple_pwm::SimplePwmChannel;
use embassy_stm32::gpio::Output;
use embassy_sync::channel::Receiver;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use crate::InputData;

#[embassy_executor::task]
pub async fn motor_controller_task(
    left_ch: SimplePwmChannel<'static, TIM1>,
    right_ch: SimplePwmChannel<'static, TIM1>,
    left_forward: Output<'static>,
    left_backward: Output<'static>,
    right_forward: Output<'static>,
    right_backward: Output<'static>,
    input_sub: Receiver<'static, NoopRawMutex, InputData, 1>
    ) {
    
}

