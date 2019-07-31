#include <stddef.h>

void* malloc(size_t sz);
void free(void* ptr);

int* get_numbers(size_t len) {
  int* x = malloc(sizeof(int)*(len));
  for (int i = 0; i < len; ++i) {
    x[i] = i;
  }
  return x;
}

void delete_numbers(int* ptr) {
  free(ptr);
}
