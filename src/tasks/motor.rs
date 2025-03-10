use embassy_stm32::peripherals::TIM1;
use embassy_stm32::timer::simple_pwm::SimplePwmChannel;
use embassy_stm32::gpio::Output;
use embassy_sync::channel::Receiver;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use crate::InputData;

#[embassy_executor::task]
pub async fn motor_controller_task(
    mut motor_l: SimplePwmChannel<'static, TIM1>,
    mut motor_r: SimplePwmChannel<'static, TIM1>,
    mut left_forward: Output<'static>,
    mut left_backward: Output<'static>,
    mut right_forward: Output<'static>,
    mut right_backward: Output<'static>,
    input_sub: Receiver<'static, NoopRawMutex, InputData, 1>
    ) {
        motor_l.enable();
        motor_r.enable();
        loop {
            let mut input = input_sub.receive().await;
            if input.y1 > 50 {
                left_forward.set_high();
                left_backward.set_low();
                right_forward.set_high();
                right_backward.set_low();
                input.y1 = (input.y1 - 50) * 2;
            } else if input.y1 < 50 {
                left_forward.set_low();
                left_backward.set_high();
                right_forward.set_low();
                right_backward.set_high();
                input.y1 = (50 - input.y1) * 2;
            } else {
                left_forward.set_low();
                left_backward.set_low();
                right_forward.set_low();
                right_backward.set_low();
            }

            motor_l.set_duty_cycle_percent(input.y1);
            motor_r.set_duty_cycle_percent(input.y1);
            _ = input.x1;
            _ = input.x2;
            _ = input.y2;
        }
}

