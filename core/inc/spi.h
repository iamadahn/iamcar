#ifndef SPI_H_
#define SPI_H_

#include "stm32f4xx_ll_spi.h"

typedef struct {
    SPI_TypeDef* spi_base;
    IRQn_Type spi_irq;
} spi_t;

void spi_config(spi_t* spi);

#endif
