#include "led.h"
//#include "led.c"

int main(void)
{
    LED_Init();

    while (1) {
        LED_SOS();
    }
}

