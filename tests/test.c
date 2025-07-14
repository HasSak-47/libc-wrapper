#include <stdio.h>

void test_0(int a, int b, int c) {
    printf("test_0: %p %p %p\n", &a, &b, &c);
    printf("test_0: %d %d %d\n", a, b, c);
}

int test_1(int a, int b, int c) {
    int r = a + b + c;
    printf("%p %p %p == %p\n", &a, &b, &c, &r);
    printf("%d %d %d == %d\n", a, b, c, r);
    return r;
}
