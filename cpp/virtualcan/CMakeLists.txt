
option(ENABLE_WINSOCK2 "Build winsock2 windows connector." OFF)
# option(ENABLE_TEST "Build unit tests." OFF)

add_library(
    virtualcan
    can_message.cpp
    frame.cpp
    util.cpp
    winsock2_connection.cpp
)

target_link_libraries(
    virtualcan
    ws2_32
)

target_include_directories(
    virtualcan
    PUBLIC
    .
)
