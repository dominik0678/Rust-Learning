#include "simple_text.h"

int text_count_vowels(const char* text) {
    int count = 0;
    int i = 0;

    if (text == 0) {
        return -1;
    }

    while (text[i] != '\0') {
        char ch = text[i];

        if (ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' ||
            ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U') {
            count++;
        }

        i++;
    }

    return count;
}

int text_count_spaces(const char* text) {
    int count = 0;
    int i = 0;

    if (text == 0) {
        return -1;
    }

    while (text[i] != '\0') {
        if (text[i] == ' ') {
            count++;
        }

        i++;
    }

    return count;
}

int text_has_digit(const char* text) {
    int i = 0;

    if (text == 0) {
        return -1;
    }

    while (text[i] != '\0') {
        if (text[i] >= '0' && text[i] <= '9') {
            return 1;
        }

        i++;
    }

    return 0;
}
