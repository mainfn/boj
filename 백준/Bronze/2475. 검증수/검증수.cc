#include <iostream>
#include <math.h>

using namespace std;

int main() {

  int sum = 0;

  for (int i = 0; i < 5; ++i) {
    int n;
    cin >> n;
    sum += pow(n, 2);
  }

  cout << sum % 10;

  return 0;
}