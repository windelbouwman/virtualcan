#ifndef VIRTUALCAN_UTIL_H
#define VIRTUALCAN_UTIL_H

#include <stdint.h>

void pack_u32(uint8_t* buffer, uint32_t value);
uint32_t unpack_u32(uint8_t* buffer);

#endif
