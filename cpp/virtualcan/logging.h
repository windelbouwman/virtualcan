#ifndef VIRTUALCAN_LOGGING_H
#define VIRTUALCAN_LOGGING_H

#include <stdio.h>

#define LOG_TRACE(msg,...) printf("TRACE:" ":" msg "\n", ##__VA_ARGS__)
#define LOG_ERROR(msg,...) printf("ERROR: at line %d :" msg "\n", __LINE__, ##__VA_ARGS__)

#endif
