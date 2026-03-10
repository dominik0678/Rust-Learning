#ifndef TEXT_TOOLS_H
#define TEXT_TOOLS_H

#include <stddef.h>

#define TXT_OK 0
#define TXT_ERR_NULL -1
#define TXT_ERR_BAD_LEN -2

int txt_count_vowels(const char* input, int* out_count);
int txt_to_upper_ascii(char* buffer, size_t len);
int txt_starts_with(const char* input, const char* prefix, int* out_result);

#endif
