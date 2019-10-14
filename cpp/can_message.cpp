
#include "can_message.h"
#include "util.h"

// serialize a can message in some format
void pack_can_msg(uint8_t* buffer, CanMessage* msg)
{
    pack_u32(buffer, msg->id);
    uint8_t flags = 0;
    if (msg->extended)
    {
        flags |= 1;
    }
    buffer[4] = flags;
    buffer[5] = msg->data_size;
    int i;
    for (i=0;i< msg->data_size;i++)
    {
        buffer[6 + i] = msg->data[i];
    }
}

void unpack_can_msg(uint8_t* buffer, CanMessage* msg)
{
    // TODO
}
