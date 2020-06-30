#ifndef VIRTUALCAN_CAN_CONNECTION_H
#define VIRTUALCAN_CAN_CONNECTION_H

#include "can_interface.h"

namespace virtualcan {

ICanConnection* open_connection(const char* host, const uint16_t port);

}

#endif
