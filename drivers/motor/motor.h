#ifndef MOTOR_H_
#define MOTOR_H_

#include "stm32f4xx_ll_tim.h"

typedef struct {
    TIM_TypeDef* tim;
    uint32_t tim_channel;
    uint8_t pwm_duty_pct;

    GPIO_TypeDef* dir_port;
    uint32_t dir_pin;

    uint8_t dir_state;
    uint8_t moving_state;
} motor_t;

void motor_init(motor_t* motor);

#endif
