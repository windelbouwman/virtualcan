
#include "unix_connection.h"
#include "logging.h"

#include <netinet/in.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <unistd.h>
#include <string.h>
#include <netdb.h>

namespace virtualcan {

UnixCanConnection::UnixCanConnection()
{
    this->socket_fd = -1;
}

UnixCanConnection::~UnixCanConnection()
{
}

int UnixCanConnection::Connect(const char* host, const uint16_t port)
{
    struct hostent *server;
    struct sockaddr_in serv_addr;
    int result;

    // Create socket:
    this->socket_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (this->socket_fd < 0) {
        LOG_ERROR("Error opening socket!");
        return -1;
    }

    // Lookup hostname:
    server = gethostbyname(host);
    if (server == NULL)
    {
        LOG_ERROR("Failed to lookup hostname: %s!", host);
        return -1;
    }

    // Connect:
    bzero(&serv_addr, sizeof(serv_addr));
    serv_addr.sin_family = AF_INET;
    bcopy(server->h_addr, &serv_addr.sin_addr.s_addr, server->h_length);
    serv_addr.sin_port = htons(port);

    result = connect(this->socket_fd, (struct sockaddr*)&serv_addr, sizeof(serv_addr));
    if (result < 0) {
        LOG_ERROR("Could not connect to server!!");
        return -1;
    }

    return 0;
}

int UnixCanConnection::tx_data(const uint8_t* buffer, const int len)
{
    int result;
    result = write(this->socket_fd, buffer, len);
    if (result < 0)
    {
        LOG_ERROR("Error transmitting data");
        return -1;
    }
    return result;
}

int UnixCanConnection::rx_data(uint8_t* buffer, const int len)
{
    int result;
    result = read(this->socket_fd, buffer, len);
    if (result < 0)
    {
        LOG_ERROR("Error receving data");
        return -1;
    }
    return result;
}

}
