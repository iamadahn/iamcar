#include "tim.h"
#include "stm32f4xx_ll_bus.h"
#include "stm32f4xx_ll_tim.h"

void
tim_pwm_init(tim_pwm_t* tim) {
    if (tim->base == TIM2) {
        LL_APB1_GRP1_EnableClock(LL_APB1_GRP1_PERIPH_TIM2);
    }

    LL_TIM_SetPrescaler(tim->base, tim->psc);
    LL_TIM_EnableARRPreload(tim->base);
    LL_TIM_SetAutoReload(tim->base, tim->arr);
    LL_TIM_OC_SetMode(tim->base, tim->channels, LL_TIM_OCMODE_PWM1);

    /* By default compare register value is set to be half of arr register value */
    LL_TIM_OC_SetCompareCH1(tim->base, (tim->arr + 1) / 2);
    LL_TIM_OC_SetCompareCH2(tim->base, (tim->arr + 1) / 2);
    LL_TIM_OC_SetCompareCH3(tim->base, (tim->arr + 1) / 2);
    LL_TIM_OC_SetCompareCH4(tim->base, (tim->arr + 1) / 2);

    LL_TIM_OC_EnablePreload(tim->base, LL_TIM_CHANNEL_CH1);
    LL_TIM_OC_EnablePreload(tim->base, LL_TIM_CHANNEL_CH2);
    LL_TIM_OC_EnablePreload(tim->base, LL_TIM_CHANNEL_CH3);
    LL_TIM_OC_EnablePreload(tim->base, LL_TIM_CHANNEL_CH4);

    LL_TIM_EnableCounter(tim->base);

    LL_TIM_GenerateEvent_UPDATE(tim->base);
}
