#pragma once

int add(int a, int b);

enum divide_error {
  OK = 0,
  DIVISION_BY_ZERO_ERROR = 1,
};

/**
 * Divides two integers and stores the result in the provided pointer.
 *
 * @param a The numerator.
 * @param b The denominator.
 * @param result Pointer to store the result.
 * @return 0 on success, non-zero on failure.
 */
enum divide_error divide(int a, int b, int *result);

struct parameter_t {
  int a;
  int b;
  int (*fun)(int a, int b);
};

struct result_t {
  struct parameter_t parameter;
  int res;
};

struct result_t process(struct parameter_t p);
