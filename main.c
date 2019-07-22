#include <stdlib.h>

int fibonacci(int n)
{
  if (n == 0 || n == 1)
    return n;
  else
    return (fibonacci(n-1) + fibonacci(n-2));
}

int main(int argc, char* argv[]) {
    int n = atoi(argv[1]);
    int r = fibonacci(n);
    printf("%d", r);
}