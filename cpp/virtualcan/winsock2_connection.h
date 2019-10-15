#ifndef VIRTUALCAN_WINSOCK2_CONNECTION_H
#define VIRTUALCAN_WINSOCK2_CONNECTION_H

#include "can_interface.h"
#include "frame.h"
#include <winsock2.h>

class WinSock2CanConnection : public ICanConnection
{
	public:
		WinSock2CanConnection();
        virtual ~WinSock2CanConnection();
		int Connect();
		int Disconnect();

		virtual void Send(CanMessage*);
		virtual CanMessage* Recv();
	
	private:
		SOCKET ConnectSocket = INVALID_SOCKET;

		int tx_packet(const Packet* packet);
		int tx_all_data(uint8_t* buffer, int len);
		int tx_data(const uint8_t* buffer, const int len);

		Packet* rx_packet();
		int rx_data(uint8_t* buffer, const int len);
		int rx_socket_exact(uint8_t* buffer, int len);
};

#endif
