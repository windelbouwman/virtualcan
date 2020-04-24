
#ifndef VIRTUALCAN_CAN_MESSAGE_H
#define VIRTUALCAN_CAN_MESSAGE_H

#include <stdint.h>

namespace virtualcan {

struct CanMessage {
    uint32_t id;
    uint32_t extended;
    uint8_t data[8];
    uint8_t data_size;
};

typedef struct CanMessage CanMessage_t;

void print_can_message(const CanMessage* can_msg);
void pack_can_msg(uint8_t* buffer, CanMessage* msg);
void unpack_can_msg(uint8_t* buffer, CanMessage* msg);

}

#endif
