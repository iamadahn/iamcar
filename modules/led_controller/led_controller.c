#include "led_controller.h"
#include "rtos.h"
#include "drivers/led/led.h"
#include "drivers/logger/logger.h"

void
led_controller_task(void* argument) {
    led_t led_debug = {
        .port = GPIOC,
        .pin = LL_GPIO_PIN_13,
        .state = 0,
    };

    while (1) {
        led_toggle(&led_debug);
        vTaskDelay(pdMS_TO_TICKS(500));
    }
}
