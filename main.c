#include <stdio.h>

int g(int i, int x, int t, int o) {
  return (
      (3 & x & (i * ((3 & i >> 16 ? "BY}6YB6$" : "Qj}6jQ6%")[t % 8] + 51) >> o))
      << 4);
}

void main() {
  for (int i = 0;; i++) {
    int n = i >> 14;
    int s = i >> 17;
    int value1 = g(i, 1, n, 12);
    int value2 = g(i, s, n ^ i >> 13, 10);
    int value3 = g(i, s / 3, n + ((i >> 11) % 3), 10);
    int value4 = g(i, s / 5, 8 + n - ((i >> 10) % 3), 9);
    int value =
      value1
      + value2
      + value3
      + value4;
    putchar(value);
  }
}
