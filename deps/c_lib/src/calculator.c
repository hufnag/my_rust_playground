#include <stdio.h>
#include <stdlib.h>

#include "c_lib/c_lib.h"

int main(int argc, char *argv[]) {
  if (argc != 4) {
    fprintf(stderr,
            "Invalid number of arguments!\nUsage: <arg1> <operator> <arg2>\n");
    return -1;
  }
  int arg1 = atoi(argv[1]);
  int arg2 = atoi(argv[3]);
  switch (*argv[2]) {
  case '+':
    printf("Result: %i\n", add(arg1, arg2));
    return 0;
  default:
    fprintf(stderr, "Invalid operator '%s'\n", argv[2]);
    return -1;
  }
  return 0;
}
