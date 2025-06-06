#ifndef VIRTUALCAN_FRAME_H
#define VIRTUALCAN_FRAME_H

#include <stdint.h>
#include <stdlib.h>

namespace virtualcan {

// A single packet
class Packet {
public:
    Packet(int length);
    virtual ~Packet();

    size_t length;
    uint8_t* data;
};

}

#endif
