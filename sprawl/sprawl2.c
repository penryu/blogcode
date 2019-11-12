/* sprawl2.c */

#include <stdint.h>
#include <stdio.h>

int main() {
    uint8_t BYTES[256][8];
    for (int str = 0; str < 256; ++str) {
        for (int bit = 0; bit < 8; ++bit) {
            BYTES[str][7 - bit] = (str & (1 << bit)) ? '1' : '0';
        }
    }

    uint8_t buf[4096];
    size_t bytes_read;
    while (0 < (bytes_read = fread(buf, sizeof(buf[0]), sizeof(buf), stdin))) {
        for (size_t i = 0; i < bytes_read; ++i) {
            fwrite(BYTES[buf[i]], 1, 8, stdout);
        }
    }
}

/* vim: set ts=4 sts=4 sw=4: */
