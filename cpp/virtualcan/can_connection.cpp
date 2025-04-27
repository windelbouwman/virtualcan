
#include "can_interface.h"

#ifdef WIN32
#include "winsock2_connection.h"
#else
#include "unix_connection.h"
#endif

namespace virtualcan {

ICanConnection* open_connection(const char* host, const uint16_t port)
{
#ifdef WIN32
    // try winsock2 connection
    WinSock2CanConnection* can_connection = new WinSock2CanConnection();
#else
    // try unix socket.
    UnixCanConnection* can_connection = new UnixCanConnection();
#endif
    can_connection->Connect(host, port);

    return can_connection;
}

}