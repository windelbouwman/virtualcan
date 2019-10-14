
cmake_minimum_required(VERSION 3.14)

project(virtualcan)

add_library(
    virtualcan
    can_message.cpp
    frame.cpp
    util.cpp
)

target_include_directories(
    virtualcan
    PUBLIC
    .
)
