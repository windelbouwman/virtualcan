/*
 * A windows winsock2 implementation.
 */

#include "winsock2_connection.h"
#include "util.h"
#include "logging.h"

#include <stdio.h>
#include <winsock2.h>
#include <ws2tcpip.h>

namespace virtualcan {

static WSADATA wsaData;

WinSock2CanConnection::WinSock2CanConnection()
{
    this->ConnectSocket = INVALID_SOCKET;
}

WinSock2CanConnection::~WinSock2CanConnection()
{
}

int WinSock2CanConnection::Connect()
{
    printf("WinSock2CanConnection::Connect\n");

    int iResult;

    // Initialize Winsock
    iResult = WSAStartup(MAKEWORD(2,2), &wsaData);
    if (iResult != 0) {
        printf("WSAStartup failed: %d\n", iResult);
        return 1;
    }

    struct addrinfo *result = NULL,
                *ptr = NULL,
                hints;

    ZeroMemory( &hints, sizeof(hints) );
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_protocol = IPPROTO_TCP;

    #define DEFAULT_PORT "18881"
    #define DEFAULT_HOST "127.0.0.1"

    // Resolve the server address and port
    iResult = getaddrinfo(DEFAULT_HOST, DEFAULT_PORT, &hints, &result);
    if (iResult != 0) {
        LOG_ERROR("getaddrinfo failed: %d", iResult);
        WSACleanup();
        return 1;
    }

    ptr = result;

    // Create a SOCKET for connecting to server
    this->ConnectSocket = socket(ptr->ai_family, ptr->ai_socktype, 
        ptr->ai_protocol);

    if (ConnectSocket == INVALID_SOCKET) {
        LOG_ERROR("Error at socket(): %ld", WSAGetLastError());
        freeaddrinfo(result);
        WSACleanup();
        return 1;
    }

    printf("Socket created!\n");

    // Connect to server.
    iResult = connect( ConnectSocket, ptr->ai_addr, (int)ptr->ai_addrlen);
    if (iResult == SOCKET_ERROR) {
        LOG_ERROR("Cannot connect!");
        closesocket(ConnectSocket);
        this->ConnectSocket = INVALID_SOCKET;
    }

    // Should really try the next address returned by getaddrinfo
    // if the connect call failed
    // But for this simple example we just free the resources
    // returned by getaddrinfo and print an error message

    freeaddrinfo(result);

    if (this->ConnectSocket == INVALID_SOCKET) {
        LOG_ERROR("Unable to connect to server!");
        WSACleanup();
        return 1;
    }

    printf("Socket connected! %d\n", this->ConnectSocket);

    return 0;
}

int WinSock2CanConnection::Disconnect()
{
    return 0;
}

// Transmit some data, preferably `len` bytes.
int WinSock2CanConnection::tx_data(const uint8_t* buffer, const int len)
{
    int iResult;

    // Check for closed socket.
    if (this->ConnectSocket == INVALID_SOCKET)
    {
        LOG_ERROR("Cannot transmit on closed socket!");
        return -1;
    }

    // Send an initial buffer
    iResult = send(this->ConnectSocket, (const char*)buffer, len, 0);
    if (iResult == SOCKET_ERROR)
    {
        LOG_ERROR("send failed: %d", WSAGetLastError());
        closesocket(this->ConnectSocket);
        WSACleanup();
        return -1;
    }

    printf("Bytes Sent: %ld\n", iResult);
    return iResult;
}

// Read some data from the socket.
// This is basically a fancy wrapper around recv from the winsock API.
int WinSock2CanConnection::rx_data(uint8_t* buffer, const int len)
{
    int iResult;

    LOG_TRACE("Trying to receive some data");

    if (this->ConnectSocket == INVALID_SOCKET)
    {
        LOG_ERROR("Cannot receive on closed socket!");
        return -1;
    }

    iResult = recv(this->ConnectSocket, (char*)buffer, len, 0);

    if (iResult > 0)
    {
        LOG_TRACE("Bytes received: %d", iResult);
        return iResult;
    }
    else if (iResult == 0)
    {
        LOG_ERROR("Connection closed");
        return -1;
    }
    else
    {
        LOG_ERROR("recv failed: %d", WSAGetLastError());
        return -1;
    }
}

}
