#include "c_lib/c_lib.h"

int add(int a, int b) { return a + b; }

enum divide_error divide(int a, int b, int *result) {
  if (b == 0) {
    return DIVISION_BY_ZERO_ERROR;
  }
  *result = a / b;
  return OK;
}

struct result_t process(struct parameter_t p) {
  struct result_t result;
  result.parameter = p;
  result.res = p.fun(p.a, p.b);
  return result;
}
