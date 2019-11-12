/* sprawl3.c */

#include <stdint.h>
#include <stdio.h>

#define BUF_SIZE 4096

int main() {
    uint8_t BYTES[256][8];
    for (int str = 0; str < 256; ++str) {
        for (int bit = 0; bit < 8; ++bit) {
            BYTES[str][7 - bit] = (str & (1 << bit)) ? '1' : '0';
        }
    }

    uint8_t inbuf[BUF_SIZE];
    uint8_t outbuf[BUF_SIZE * 8];
    size_t bytes;
    while (0 < (bytes = fread(inbuf, sizeof(inbuf[0]), sizeof(inbuf), stdin))) {
        for (size_t b = 0; b < bytes; ++b) {
            for (size_t s = 0; s < 8; ++s) {
                outbuf[b * 8 + s] = BYTES[inbuf[b]][s];
            }
        }
        fwrite(outbuf, sizeof(outbuf[0]), (bytes * sizeof(BYTES[0])), stdout);
    }
}

/* vim: set ts=4 sts=4 sw=4: */
