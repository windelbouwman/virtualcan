#include "base_connection.h"
#include "logging.h"
#include "util.h"

namespace virtualcan
{

BaseCanConnection::~BaseCanConnection()
{
}

void BaseCanConnection::Send(CanMessage* msg)
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

CanMessage* BaseCanConnection::Recv()
{
    Packet* packet = rx_packet();
    LOG_TRACE("Got packet!");
    CanMessage* msg = new CanMessage();
    unpack_can_msg(packet->data, msg);
    delete packet;
    return msg;
}


// Transmit a whole packet
int BaseCanConnection::tx_packet(const Packet* packet)
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

Packet* BaseCanConnection::rx_packet()
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

// Try hard to emit all data
int BaseCanConnection::tx_all_data(uint8_t* buffer, int len)
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

// Read exactly a certain amount of bytes from the socket.
int BaseCanConnection::rx_socket_exact(uint8_t* buffer, int len)
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

}