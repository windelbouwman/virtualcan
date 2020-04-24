
#ifndef VIRTUALCAN_CAN_INTERFACE_H
#define VIRTUALCAN_CAN_INTERFACE_H

#include "can_message.h"

namespace virtualcan {

class ICanConnection
{
	public:
		virtual void Send(CanMessage*) = 0;
		virtual CanMessage* Recv() = 0;
};

}

#endif
