
#ifndef UNIX_CAN_CONNECTION_H
#define UNIX_CAN_CONNECTION_H

#include "base_connection.h"
#include <sys/socket.h>

namespace virtualcan {

class UnixCanConnection : public BaseCanConnection {
    public:
        UnixCanConnection();
        virtual ~UnixCanConnection();
        int Connect(const char* host, const uint16_t port);

    protected:
		virtual int tx_data(const uint8_t* buffer, const int len);
		virtual int rx_data(uint8_t* buffer, const int len);

    private:
        int socket_fd;
};

}

#endif
