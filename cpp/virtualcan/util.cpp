
#include "util.h"

namespace virtualcan {

// pack uint32_t as big endian into a buffer
void pack_u32(uint8_t* buffer, uint32_t value)
{
    int i;
    int shift;

    for (i=0;i<4;i++)
    {
        shift = 24 - (i * 8);
        buffer[i] = (value >> shift) & 0xff;
    }
}

uint32_t unpack_u32(uint8_t* buffer)
{
    uint32_t value = 0;

    int i;

    for (i=0;i<4;i++)
    {
        value <<= 8;
        value |= buffer[i];
    }

    return value;
}

}
