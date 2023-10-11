#include <iostream>

using namespace std;

int main() {

  int sum = 0;
  string numstr;
  cin >> numstr >> numstr;

  for (int i = 0; i < numstr.length(); ++i) {
    sum += numstr[i] - '0';
  }

  cout << sum;

  return 0;
}
