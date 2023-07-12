
#include <cstdio>

struct S1 {
  S1() { std::puts("S1()"); }
};

S1 s1;

extern "C" {
void f() {}
}
