
#ifndef VIRTUALCAN_CAN_MESSAGE_H
#define VIRTUALCAN_CAN_MESSAGE_H

#include <stdint.h>

struct CanMessage {
    uint32_t id;
    uint32_t extended;
    uint8_t data[8];
    uint8_t data_size;
};

typedef struct CanMessage CanMessage_t;

#endif
