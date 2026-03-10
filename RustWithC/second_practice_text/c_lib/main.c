#include "text_tools.h"

#include <stdio.h>

int main(void) {
    char text[] = "Rust and C together";
    int vowels = 0;
    int starts_with = 0;
    int status = 0;

    printf("text_tools C demo\n");

    status = txt_count_vowels(text, &vowels);
    printf("txt_count_vowels -> status=%d, vowels=%d\n", status, vowels);

    status = txt_to_upper_ascii(text, sizeof(text));
    printf("txt_to_upper_ascii -> status=%d, result=\"%s\"\n", status, text);

    status = txt_starts_with(text, "RUST", &starts_with);
    printf("txt_starts_with(\"RUST\") -> status=%d, result=%d\n", status, starts_with);

    return 0;
}
