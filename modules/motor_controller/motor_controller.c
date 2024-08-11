#include "motor_controller.h"
#include "rtos.h"
#include "drivers/motor/motor.h"
#include "stm32f4xx_ll_gpio.h"

void
motor_controller_task(void* argument) {
    motor_t motor_left = {
        .tim = TIM2,
        .tim_channel = LL_TIM_CHANNEL_CH2,
        .dir_port = GPIOA,
        .dir_pin = LL_GPIO_PIN_3,
    };
    
    while (1) {
      vTaskDelay(pdMS_TO_TICKS(5));
    }
}
