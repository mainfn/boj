#include <iostream>

using namespace std;

int main() {
  int a, b, height;

  cin >> a >> b >> height;

  int total_days = (height - a) / (a - b);

  if ((height - a) % (a - b) != 0) {
    total_days += 2;
  } else {
    total_days += 1;
  }
  cout << total_days;

  return 0;
}
