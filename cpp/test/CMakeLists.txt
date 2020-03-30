
add_executable(
    virtualcan_tests
    main.cpp
    test_packing.cpp
)

target_link_libraries(
    virtualcan_tests
    virtualcan
    gtest
)

target_include_directories(
    virtualcan_tests
    PRIVATE
    "C:/Program Files (x86)/googletest-distribution/include"
)

target_link_directories(
    virtualcan_tests
    PRIVATE
    "C:/Program Files (x86)/googletest-distribution/lib"
)
