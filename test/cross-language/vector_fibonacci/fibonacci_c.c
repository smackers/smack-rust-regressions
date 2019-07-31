unsigned long fib_c(unsigned long x) {
  unsigned long a = 0, b = 1;
  for (unsigned long i = 0; i < x-1; i++) {
    unsigned long tmp = a;
    a = b;
    b = a + tmp;
  }
  return b;
}
