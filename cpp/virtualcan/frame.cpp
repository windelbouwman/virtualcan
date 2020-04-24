
#include "frame.h"
#include <stdlib.h>

namespace virtualcan {

// Destructor will free the associated data
Packet::~Packet() {
    free(this->data);
}

Packet::Packet(int length)
{
    this->data = (uint8_t*)malloc(length);
    this->length = length;
}

}
