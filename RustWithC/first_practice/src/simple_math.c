#include "simple_math.h"

int math_add(int a, int b) {
    return a + b;
}

int math_sub(int a, int b) {
    return a - b;
}

int math_mul(int a, int b) {
    return a * b;
}

int math_abs(int x) {
    if (x < 0) {
        return -x;
    }

    return x;
}

int math_max(int a, int b) {
    if (a > b) {
        return a;
    }

    return b;
}
