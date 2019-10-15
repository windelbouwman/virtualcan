
#include <assert.h>
#include "util.h"
#include "can_message.h"

void test_u32_packing()
{
    uint8_t buf1[4];
    pack_u32(buf1, 1337);
    assert(1337 == unpack_u32(buf1));
}


void test_can_packing()
{
    CanMessage* msg1 = new CanMessage();
    msg1->id = 1338;
    msg1->extended = 0;
    msg1->data_size = 2;
    msg1->data[0] = 13;
    msg1->data[1] = 65;
    uint8_t buf1[14];
    pack_can_msg(buf1, msg1);
    delete msg1;

    CanMessage* msg2 = new CanMessage();
    unpack_can_msg(buf1, msg2);
    assert(1338 == msg2->id);
    assert(0 == msg2->extended);
    assert(2 == msg2->data_size);
    assert(13 == msg2->data[0]);
    assert(65 == msg2->data[1]);

    delete msg2;
}

