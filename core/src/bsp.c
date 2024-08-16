#include "bsp.h"
#include "rcc.h"
#include "stm32f411xe.h"
#include "stm32f4xx_ll_bus.h"
#include "stm32f4xx_ll_gpio.h"
#include "stm32f4xx_ll_utils.h"
#include "stm32f4xx_ll_tim.h"
#include "tim.h"
#include "spi.h"

void
bsp_init(void) {

    /* Configure RCC to use HSE and get 96 MHz */
    rcc_config();

    /* Configure NVIC for FreeRTOS to work correctly */
    NVIC_SetPriorityGrouping(3U);
    NVIC_SetPriority(SysTick_IRQn, NVIC_EncodePriority(NVIC_GetPriorityGrouping(), 15, 0));

    /* GPIOA port clock enable */
    LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_GPIOA);

    /* TIM2 pins initalisation */
    LL_GPIO_InitTypeDef tim2_pins = {
        .Pin = LL_GPIO_PIN_1 | LL_GPIO_PIN_2,
        .Mode = LL_GPIO_MODE_ALTERNATE,
        .Speed = LL_GPIO_SPEED_FREQ_VERY_HIGH,
        .Pull = LL_GPIO_PULL_DOWN,
        .Alternate = LL_GPIO_AF_1,
    };
    LL_GPIO_Init(GPIOA, &tim2_pins);

    /* TIM2 pwm mode initialisation */
    tim_pwm_t tim2_pwm = {
        .base = TIM2,
        .channels = LL_TIM_CHANNEL_CH2 | LL_TIM_CHANNEL_CH3,
        .psc = 959, // 96000000 Hz / (959 + 1) = 100000 Hz
        .arr = 99,
    };
    tim_pwm_init(&tim2_pwm);

    /* SPI1 pins initialisation */
    LL_GPIO_InitTypeDef spi1_pins = {
        .Pin = LL_GPIO_PIN_5 | LL_GPIO_PIN_7,
        .Mode = LL_GPIO_MODE_ALTERNATE,
        .Speed = LL_GPIO_SPEED_FREQ_VERY_HIGH,
        .OutputType = LL_GPIO_OUTPUT_PUSHPULL,
        .Alternate = LL_GPIO_AF_5,
    };
    LL_GPIO_Init(GPIOA, &spi1_pins);

    spi1_pins.Pin = LL_GPIO_PIN_6;
    spi1_pins.Mode = LL_GPIO_MODE_INPUT;
    LL_GPIO_Init(GPIOA, &spi1_pins);

    /* SPI1 initialisation */
    spi_t spi1 = {
        .spi_base = SPI1,
        .spi_irq = SPI1_IRQn,
    };
    spi_config(&spi1);
    
    /* USB_OTG pins initialisation */
    LL_GPIO_InitTypeDef usb_pins = {
		.Pin = LL_GPIO_PIN_8 | LL_GPIO_PIN_11 | LL_GPIO_PIN_12,
		.Mode = LL_GPIO_MODE_ALTERNATE,
		.Speed = LL_GPIO_SPEED_FREQ_VERY_HIGH,
		.OutputType = LL_GPIO_OUTPUT_PUSHPULL,
		.Pull = LL_GPIO_PULL_NO,
		.Alternate = LL_GPIO_AF_10
	};
    LL_GPIO_Init(GPIOA, &usb_pins);
	
	NVIC_SetPriority(OTG_FS_IRQn, 5);
	LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_OTGFS);

	/* Literally the most important line in configuring USB_OTG ever */
    USB_OTG_FS->GCCFG |= USB_OTG_GCCFG_NOVBUSSENS;
    USB_OTG_FS->GCCFG &= ~USB_OTG_GCCFG_VBUSBSEN;
    USB_OTG_FS->GCCFG &= ~USB_OTG_GCCFG_VBUSASEN;

    /* Enable LED on GPIOC pin 13 */
    LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_GPIOC);
    LL_GPIO_SetPinMode(GPIOC, LL_GPIO_PIN_13, LL_GPIO_MODE_OUTPUT);

    /* Enable CE and CSN for NRF24L01*/
    LL_AHB1_GRP1_EnableClock(LL_AHB1_GRP1_PERIPH_GPIOB);
    LL_GPIO_SetPinMode(GPIOB, LL_GPIO_PIN_0, LL_GPIO_MODE_OUTPUT);
    LL_GPIO_SetPinMode(GPIOB, LL_GPIO_PIN_1, LL_GPIO_MODE_OUTPUT);

    /* Enable CE for W25Q and set it to high lvl */
    LL_GPIO_SetPinMode(GPIOA, LL_GPIO_PIN_4, LL_GPIO_MODE_OUTPUT);
    LL_GPIO_SetOutputPin(GPIOA, LL_GPIO_PIN_4);
}
