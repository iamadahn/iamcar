#ifndef U_RTOS_H_
#define U_RTOS_H_

#include "FreeRTOS.h"
#include "FreeRTOSConfig.h"
#include "task.h"
#include "queue.h"

extern QueueHandle_t nrf24_state_queue;

void threads_init(void);
void queues_init(void);
void kernel_start(void);

void led_controller_task(void* pv_argument);
void rc_controller_task(void* pv_argument);

#endif
