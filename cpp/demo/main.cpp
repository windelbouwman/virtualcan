
#include <stdio.h>
#include "can_connection.h"

int main()
{
    printf("Demo virtual can usage\n");

    virtualcan::ICanConnection* can_connection = virtualcan::open_connection();

    virtualcan::CanMessage* msg = new virtualcan::CanMessage();
    msg->id = 1337;
    msg->data[0] = 1;
    msg->data[1] = 13;
    msg->extended = 0;
    msg->data_size = 2;

    print_can_message(msg);
    can_connection->Send(msg);

    virtualcan::CanMessage* msg2 = can_connection->Recv();
    virtualcan::print_can_message(msg2);
    delete msg2;

    // receive some messages:
    int i;
    for (i=0;i<20;i++)
    {
        msg2 = can_connection->Recv();
        print_can_message(msg2);
        delete msg2;
    }
}
