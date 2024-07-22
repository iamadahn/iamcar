#include "rtos.h"
#include "modules/led_controller/led_controller.h"
#include "modules/rc_controller/rc_controller.h"

QueueHandle_t nrf24_state_queue;

void
delay_ms(uint32_t ms) {
    vTaskDelay(pdMS_TO_TICKS(ms));
}

void
threads_init(void) {
    xTaskCreate(led_controller_task, "led_controller", configMINIMAL_STACK_SIZE, NULL, tskIDLE_PRIORITY, NULL);

    //xTaskCreate(rc_controller_task, "rc_controller", configMINIMAL_STACK_SIZE * 2, NULL, configMAX_PRIORITIES - 1U, NULL);
}

void
queues_init(void) {
    nrf24_state_queue = xQueueCreate(1, sizeof(uint8_t));
}

void
kernel_start(void) {
    vTaskStartScheduler();
}

void
led_controller_task(void* pv_argument) {
    led_controller_handler();
}

void
rc_controller_task(void* pv_argument) {
    rc_controller_task_handler();
}

#if ( configCHECK_FOR_STACK_OVERFLOW > 0 )

    void vApplicationStackOverflowHook( TaskHandle_t xTask,
                                        char * pcTaskName )
    {
        /* Check pcTaskName for the name of the offending task,
         * or pxCurrentTCB if pcTaskName has itself been corrupted. */
        ( void ) xTask;
        ( void ) pcTaskName;
    }

#endif /* #if ( configCHECK_FOR_STACK_OVERFLOW > 0 ) */
