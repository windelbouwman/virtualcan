cmake_minimum_required(VERSION 3.20.0)
project(virtualcan VERSION 0.1.0 LANGUAGES C CXX)


if (WIN32)
    set(CONNECTION_IMPLEMENTATION_SRC winsock2_connection.cpp)
else()
    set(CONNECTION_IMPLEMENTATION_SRC unix_connection.cpp)
endif()


add_library(virtualcan SHARED)
target_sources(virtualcan
    PRIVATE
        base_connection.cpp
        can_connection.cpp
        can_message.cpp
        frame.cpp
        util.cpp
        ${CONNECTION_IMPLEMENTATION_SRC}
    PUBLIC
    FILE_SET HEADERS FILES
        can_connection.h
)

if (WIN32)
    target_link_libraries(
        virtualcan
        ws2_32
    )
endif()


include(CMakePackageConfigHelpers)
configure_package_config_file(cmake/virtualcanConfig.cmake.in
    ${CMAKE_CURRENT_BINARY_DIR}/virtualcanConfig.cmake
    INSTALL_DESTINATION ${CMAKE_INSTALL_PREFIX}/cmake/virtualcan
)

write_basic_package_version_file(
    ${CMAKE_CURRENT_BINARY_DIR}/virtualcanConfigVersion.cmake
    VERSION ${PROJECT_VERSION}
    COMPATIBILITY ExactVersion
)

install(FILES ${CMAKE_CURRENT_BINARY_DIR}/virtualcanConfig.cmake
              ${CMAKE_CURRENT_BINARY_DIR}/virtualcanConfigVersion.cmake
        DESTINATION ${CMAKE_INSTALL_PREFIX}/cmake/virtualcan
)

install(TARGETS virtualcan
        FILE_SET HEADERS DESTINATION "${CMAKE_INSTALL_PREFIX}/include"
)