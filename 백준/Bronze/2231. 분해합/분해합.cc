#include <iostream>

using namespace std;

bool IsNumberConstructor(int number, int number_constructor) {
  int sum = number_constructor;

  while (number_constructor > 0) {
    int unit = number_constructor % 10;
    number_constructor /= 10;
    sum += unit;
  }

  return sum == number;
}

int main() {
  int n;
  cin >> n;

  for (int i = 1; i <= 1000000; i++) {
    if (IsNumberConstructor(n, i)) {
      cout << i;
      return 0;
    }
  }

  cout << 0;
  return 0;
}