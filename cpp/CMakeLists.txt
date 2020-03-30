
cmake_minimum_required(VERSION 3.12)
project(virtualcan)

option(BUILD_TESTS "Build unit tests" OFF)

add_subdirectory(virtualcan)
add_subdirectory(demo)

if(BUILD_TESTS)
  add_subdirectory(test)
endif(BUILD_TESTS)
