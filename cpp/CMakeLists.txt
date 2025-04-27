
cmake_minimum_required(VERSION 3.12)
project(virtualcan LANGUAGES CXX)

option(BUILD_TESTS "Build unit tests")

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_subdirectory(virtualcan)
add_subdirectory(demo)

if(BUILD_TESTS)
  add_subdirectory(test)
endif(BUILD_TESTS)
