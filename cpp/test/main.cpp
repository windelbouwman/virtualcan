
#include <stdio.h>

// TODO: use proper unittest framework.
void test_u32_packing();
void test_can_packing();

int main()
{
    printf("Running tests!\n");
    test_u32_packing();
    test_can_packing();
    printf("Success!\n");
    return 1;
}
