
#include <gtest/gtest.h>
#include "util.h"
#include "can_message.h"

using namespace virtualcan;

TEST(VirtualCanTest, TestU32Packing)
{
    uint8_t buf1[4];
    pack_u32(buf1, 1337);
    ASSERT_EQ(1337, unpack_u32(buf1));
}

TEST(VirtualCanTest, TestCanPacking)
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
    ASSERT_EQ(1338, msg2->id);
    ASSERT_EQ(0, msg2->extended);
    ASSERT_EQ(2, msg2->data_size);
    ASSERT_EQ(13, msg2->data[0]);
    ASSERT_EQ(65, msg2->data[1]);

    delete msg2;
}

