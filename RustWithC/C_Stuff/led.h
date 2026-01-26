#ifndef LED_H
#define LED_H

#include <stdint.h>

/*
 * Simple LED library for STM32G474 (bare-metal, no HAL).
 * Default: PA5 (Nucleo LD2). You can change to PA4 in led.c.
 */

void LED_Init(void);
void LED_On(void);
void LED_Off(void);

void LED_Blink(uint32_t times, uint32_t delay_ms);
void LED_SOS(void);

#endif /* LED_H */
