
#ifndef VIRTUALCAN_BASE_CONNECTION_H
#define VIRTUALCAN_BASE_CONNECTION_H

#include "can_interface.h"
#include "frame.h"

class BaseCanConnection : public ICanConnection
{
	public:
        virtual ~BaseCanConnection();

		virtual void Send(CanMessage*);
		virtual CanMessage* Recv();

    protected:
		virtual int tx_data(const uint8_t* buffer, const int len) = 0;
		virtual int rx_data(uint8_t* buffer, const int len) = 0;

	private:
		int tx_packet(const Packet* packet);
		int tx_all_data(uint8_t* buffer, int len);

		Packet* rx_packet();
		int rx_socket_exact(uint8_t* buffer, int len);
};

#endif
