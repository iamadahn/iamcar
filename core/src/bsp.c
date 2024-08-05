#include "bsp.h"
#include "rcc.h"
#include "stm32f4xx_ll_bus.h"
#include "stm32f4xx_ll_gpio.h"
#include "stm32f4xx_ll_utils.h"
#include "stm32f4xx_ll_tim.h"
#include <tim.h>

void
bsp_init(void) {

    /* Configure RCC to use HSE and get 96 MHz */
    rcc_config();

    /* Configure NVIC for FreeRTOS to work correctly */
    NVIC_SetPriorityGrouping(3U);
    NVIC_SetPriority(SysTick_IRQn, NVIC_EncodePriority(NVIC_GetPriorityGrouping(), 15, 0));

    tim_pwm_t tim2_pwm = {
        .base = TIM2,
        .port = GPIOA,
        .pins = LL_GPIO_PIN_1 | LL_GPIO_PIN_2,
        .channels = LL_TIM_CHANNEL_CH2 | LL_TIM_CHANNEL_CH3,
        .alt_func = LL_GPIO_AF_1,
        .psc = 1,
        .arr = 1,
    };
    tim_pwm_init(&tim2_pwm);

    /* Create SPI struct and configure chosen SPI */
    /*
    spi_t spi1 = {
        .spi_base = SPI1,
        .sck_port = GPIOA,
        .sck_pin = LL_GPIO_PIN_5,
        .mosi_port = GPIOA,
        .mosi_pin = LL_GPIO_PIN_7,
        .miso_port = GPIOA,
        .miso_pin = LL_GPIO_PIN_6,
        .spi_irq = SPI1_IRQn,
    };
    spi_config(&spi1);
    */

    /* Enable LED on GPIOB pin 2 */
    LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_GPIOC);
    LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_13, LL_GPIO_MODE_OUTPUT);

    /* Enable CE and CSN for NRF24L01*/
    //LL_APB2_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_GPIOC);
    //LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_13, LL_GPIO_MODE_OUTPUT);
    //LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_14, LL_GPIO_MODE_OUTPUT);

    /* Enable CE for W25Q and set it to high lvl */
    LL_APB2_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_GPIOA);
    LL_GPIO_SetPinMode(GPIOA, LL_GPIO_PIN_4, LL_GPIO_MODE_OUTPUT);
    LL_GPIO_SetOutputPin(GPIOA, LL_GPIO_PIN_4);
}
