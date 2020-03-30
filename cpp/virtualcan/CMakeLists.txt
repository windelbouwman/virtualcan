

if (WIN32)
    set(CONNECTION_IMPLEMENTATION_SRC winsock2_connection.cpp)
else()
    set(CONNECTION_IMPLEMENTATION_SRC unix_connection.cpp)
endif()

add_library(
    virtualcan
    base_connection.cpp
    can_connection.cpp
    can_message.cpp
    frame.cpp
    util.cpp
    ${CONNECTION_IMPLEMENTATION_SRC}
)

if (WIN32)
    target_link_libraries(
        virtualcan
        ws2_32
    )
endif()

target_include_directories(
    virtualcan
    PUBLIC
    .
)
