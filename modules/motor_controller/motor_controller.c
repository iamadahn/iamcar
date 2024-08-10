#include "motor_controller.h"
#include "rtos.h"

void
motor_controller_task(void* argument) {
    while (1) {
      vTaskDelay(pdMS_TO_TICKS(5));
    }
}
