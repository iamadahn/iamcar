#ifndef TIM_H_
#define TIM_H_

#include <stdint.h>
#include "stm32f411xe.h"

typedef struct {
  TIM_TypeDef* base;
  GPIO_TypeDef* port;
  uint32_t pins;
  uint32_t channels;
  uint32_t alt_func;
  uint32_t psc;
  uint32_t arr;
} tim_pwm_t;

void tim_pwm_init(tim_pwm_t* tim);

#endif
