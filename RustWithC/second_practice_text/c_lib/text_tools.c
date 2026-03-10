#include "text_tools.h"

static int is_vowel_ascii(char ch) {
    switch (ch) {
        case 'a':
        case 'e':
        case 'i':
        case 'o':
        case 'u':
        case 'A':
        case 'E':
        case 'I':
        case 'O':
        case 'U':
            return 1;
        default:
            return 0;
    }
}

int txt_count_vowels(const char* input, int* out_count) {
    size_t i = 0;
    int count = 0;

    if (input == NULL || out_count == NULL) {
        return TXT_ERR_NULL;
    }

    while (input[i] != '\0') {
        if (is_vowel_ascii(input[i])) {
            count++;
        }
        i++;
    }

    *out_count = count;
    return TXT_OK;
}

int txt_to_upper_ascii(char* buffer, size_t len) {
    size_t i = 0;
    int found_terminator = 0;

    if (buffer == NULL) {
        return TXT_ERR_NULL;
    }

    if (len == 0) {
        return TXT_ERR_BAD_LEN;
    }

    while (i < len) {
        char ch = buffer[i];

        if (ch == '\0') {
            found_terminator = 1;
            break;
        }

        if (ch >= 'a' && ch <= 'z') {
            buffer[i] = (char)(ch - ('a' - 'A'));
        }

        i++;
    }

    if (!found_terminator) {
        return TXT_ERR_BAD_LEN;
    }

    return TXT_OK;
}

int txt_starts_with(const char* input, const char* prefix, int* out_result) {
    size_t i = 0;

    if (input == NULL || prefix == NULL || out_result == NULL) {
        return TXT_ERR_NULL;
    }

    while (prefix[i] != '\0') {
        if (input[i] == '\0' || input[i] != prefix[i]) {
            *out_result = 0;
            return TXT_OK;
        }

        i++;
    }

    *out_result = 1;
    return TXT_OK;
}
