static inline long __internal_syscall(long n, long _a0, long _a1, long _a2,
                                      long _a3, long _a4, long _a5) {
  register long a0 asm("a0") = _a0;
  register long a1 asm("a1") = _a1;
  register long a2 asm("a2") = _a2;
  register long a3 asm("a3") = _a3;
  register long a4 asm("a4") = _a4;
  register long a5 asm("a5") = _a5;
  register long syscall_id asm("a7") = n;

  asm volatile("scall"
               : "+r"(a0)
               : "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5), "r"(syscall_id));
  return a0;
}

#define syscall(n, a, b, c, d, e, f)                                           \
  __internal_syscall(n, (long)(a), (long)(b), (long)(c), (long)(d), (long)(e), \
                     (long)(f))

int main() {
    int argc = 3;
    char *argv[] = {"a", "b", "c"};
    int ret = syscall(2043, 1, 3, 0, 0xffffffffffffffff, argc, argv);
    if (ret != 0) {
      return ret;
    }
    return -1;
}
