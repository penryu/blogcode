/* sprawl1.c */

#include <stdint.h>                                                                                       
#include <stdio.h>                                                                                        
                                                                                                          
static const char *BYTES[256] = {                                                                         
    "00000000", "00000001", "00000010", "00000011",                                                   
    "00000100", "00000101", "00000110", "00000111",                                                   
    "00001000", "00001001", "00001010", "00001011",                                                   
  /* ... ALL THE BITSTRINGS ... */                                                                        
    "11111100", "11111101", "11111110", "11111111",                                                   
};                                                                                                        
                                                                                                          
int main() {                                                                                              
    uint8_t buf[4096];                                                                                
    size_t bytes_read;                                                                                
                                                                                                      
    while (0 < (bytes_read = fread(buf, sizeof(buf[0]), sizeof(buf), stdin))) {                       
        for (size_t i = 0; i < bytes_read; ++i) {                                                 
            printf("%s", BYTES[buf[i]]);                                                      
        }                                                                                         
    }                                                                                                 
}                                                                                                         

/* vim: set ts=4 sts=4 sw=4: */
