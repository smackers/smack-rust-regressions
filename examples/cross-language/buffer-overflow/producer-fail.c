#include <stddef.h>

int* get_numbers(size_t len) {
  int* x = malloc(sizeof(int)*(len-1));
  for (int i = 0; i < len-1; ++i) {
    x[i] = i;
  }
  return x;
}
