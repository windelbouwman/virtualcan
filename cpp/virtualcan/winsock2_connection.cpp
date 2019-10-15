/*
 * A windows winsock2 implementation.
 */

#include "winsock2_connection.h"
#include "util.h"

#include <stdio.h>
#include <winsock2.h>
#include <ws2tcpip.h>

static WSADATA wsaData;

#define LOG_TRACE(msg,...) printf("TRACE:" ":" msg "\n", ##__VA_ARGS__)
#define LOG_ERROR(msg,...) printf("ERROR: at line %d :" msg "\n", __LINE__, ##__VA_ARGS__)

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

    #define DEFAULT_PORT "8888"
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

void WinSock2CanConnection::Send(CanMessage* msg)
{
    Packet* packet = new Packet(14);
    pack_can_msg(packet->data, msg);
    delete msg;

    printf("Dikke vette send actie\n");

    int result;
    result = this->tx_packet(packet);

    if (result < 0)
    {
        LOG_ERROR("Error in transmission of can message");
    }
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

// Try hard to emit all data
int WinSock2CanConnection::tx_all_data(uint8_t* buffer, int len)
{
    int result;
    while (len > 0)
    {
        result = tx_data(buffer, len);
        if (result <= 0)
        {
            LOG_ERROR("Transmission of data chunk failed");
            return -1;
        }

        len -= result;
        buffer += result;
    }

    return 0;
}

// Transmit a whole packet
int WinSock2CanConnection::tx_packet(const Packet* packet)
{
    int result;

    // tx frame length
    uint8_t frame_length_buffer[4];
    pack_u32(frame_length_buffer, packet->length);
    result = tx_all_data(frame_length_buffer, 4);
    if (result < 0)
    {
        LOG_ERROR("Transmission of data failed");
        return -1;
    }

    // tx frame data:
    result = tx_all_data(packet->data, packet->length);
    if (result < 0)
    {
        LOG_ERROR("Transmission of data failed");
        return -1;
    }

    delete packet;

    return 0;
}


Packet* WinSock2CanConnection::rx_packet()
{
    uint8_t packet_length_buffer[4];
    rx_socket_exact(packet_length_buffer, 4);

    uint32_t packet_length = unpack_u32(packet_length_buffer);
    printf("Packet length: %d\n", packet_length);

    // Create new packet of given size:
    Packet* packet = new Packet(packet_length);
    rx_socket_exact(packet->data, packet_length);
    return packet;
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

// Read exactly a certain amount of bytes from the socket.
int WinSock2CanConnection::rx_socket_exact(uint8_t* buffer, int len)
{
    int bytes_read;
    while (len > 0)
    {
        bytes_read = this->rx_data(buffer, len);
        if (bytes_read <= 0) {
            return -1;
        }

        buffer += bytes_read;
        len -= bytes_read;
    }

    return 0;
}

CanMessage* WinSock2CanConnection::Recv()
{
    Packet* packet = rx_packet();
    LOG_TRACE("Got packet!");
    CanMessage* msg = new CanMessage();
    unpack_can_msg(packet->data, msg);
    delete packet;
    return msg;
}
