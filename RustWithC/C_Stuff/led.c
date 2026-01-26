#include "led.h"

/* --- Minimal register definitions (no CMSIS needed) --- */
#define PERIPH_BASE             0x40000000UL
#define AHB2PERIPH_BASE         (PERIPH_BASE + 0x08000000UL)

#define GPIOA_BASE              (AHB2PERIPH_BASE + 0x0000UL)

/* RCC ist NICHT im AHB2-Block */
#define RCC_BASE                0x40021000UL
#define RCC_AHB2ENR             (*(volatile uint32_t *)(RCC_BASE + 0x4CUL))

/* GPIOA */
#define GPIOA_MODER             (*(volatile uint32_t *)(GPIOA_BASE + 0x00UL))
#define GPIOA_OTYPER            (*(volatile uint32_t *)(GPIOA_BASE + 0x04UL))
#define GPIOA_OSPEEDR           (*(volatile uint32_t *)(GPIOA_BASE + 0x08UL))
#define GPIOA_PUPDR             (*(volatile uint32_t *)(GPIOA_BASE + 0x0CUL))
#define GPIOA_BSRR              (*(volatile uint32_t *)(GPIOA_BASE + 0x18UL))

/* --- Choose pin here --- */
/* Nucleo-G474RE onboard LED (LD2) is typically PA5 */
#define LED_PIN_A 5U
#define LED_PIN_B 4U

static void delay_cycles(volatile uint32_t cycles)
{
    while (cycles--) {
        __asm volatile ("nop");
    }
}

/*
 * Very rough delay:
 * - depends on core clock
 * - good enough for "blink a LED" demos
 */
static void delay_ms(uint32_t ms)
{
    /* crude scale factor; adjust if you care */
    while (ms--) {
        delay_cycles(8000U);
    }
}

void LED_Init(void)
{
    /* Enable GPIOA clock */
    RCC_AHB2ENR |= (1U << 0);

    /* Set output type push-pull */
    GPIOA_OTYPER &= ~(1U << LED_PIN_A); // Onboward LED
    GPIOA_OTYPER &= ~(1U << LED_PIN_B); // LED on Port A2

    /* No pull-up/down */
    GPIOA_PUPDR &= ~(3U << (LED_PIN_A * 2U));
    GPIOA_PUPDR &= ~(3U << (LED_PIN_B * 2U));

    /* Medium speed (optional, but typical) */
    GPIOA_OSPEEDR &= ~(3U << (LED_PIN_A * 2U));
    GPIOA_OSPEEDR |=  (1U << (LED_PIN_A * 2U));
    GPIOA_OSPEEDR &= ~(3U << (LED_PIN_B * 2U));
    GPIOA_OSPEEDR |=  (1U << (LED_PIN_B * 2U));

    /* Set as general purpose output: MODER = 01 */
    GPIOA_MODER &= ~(3U << (LED_PIN_A * 2U));
    GPIOA_MODER |=  (1U << (LED_PIN_A * 2U));
    GPIOA_MODER &= ~(3U << (LED_PIN_B * 2U));
    GPIOA_MODER |=  (1U << (LED_PIN_B * 2U));

    /* Start OFF */
    LED_Off();
}

void LED_On(void)
{
    /* BSRR lower 16 bits set pin */
    GPIOA_BSRR = (1U << LED_PIN_A) | (1U << LED_PIN_B);
}

void LED_Off(void)
{
    /* BSRR upper 16 bits reset pin */
    GPIOA_BSRR = (1U << (LED_PIN_A + 16U)) | (1U << (LED_PIN_B + 16U));
}

void LED_Blink(uint32_t times, uint32_t delay_ms_val)
{
    uint32_t i;
    for (i = 0; i < times; i++) {
        LED_On();
        delay_ms(delay_ms_val);
        LED_Off();
        delay_ms(delay_ms_val);
    }
}

/* Morse timing units */
static void dot(void)
{
    LED_On();
    delay_ms(15);
    LED_Off();
    delay_ms(15);
}

static void dash(void)
{
    LED_On();
    delay_ms(45);
    LED_Off();
    delay_ms(45);
}

static void letter_gap(void)
{
    delay_ms(30);
}

static void word_gap(void)
{
    delay_ms(90);
}

void LED_SOS(void)
{
    /* S: ... */
    dot(); dot(); dot();
    letter_gap();

    /* O: --- */
    dash(); dash(); dash();
    letter_gap();

    /* S: ... */
    dot(); dot(); dot();

    word_gap();
}
