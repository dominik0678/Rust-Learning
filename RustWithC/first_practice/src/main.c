#include "simple_math.h"

#include <stdio.h>

int main(void) {
    int a = 20;
    int b = 6;

    printf("simple_math C demo\n");
    printf("math_add(%d, %d) = %d\n", a, b, math_add(a, b));
    printf("math_sub(%d, %d) = %d\n", a, b, math_sub(a, b));
    printf("math_mul(%d, %d) = %d\n", a, b, math_mul(a, b));
    printf("math_abs(%d) = %d\n", -42, math_abs(-42));
    printf("math_max(%d, %d) = %d\n", a, b, math_max(a, b));

    return 0;
}
