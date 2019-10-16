
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
