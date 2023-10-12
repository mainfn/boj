#include <iostream>

using namespace std;

int main() {
  int a, b, c;

  cin >> a >> b >> c;
  string mul = to_string(a * b * c);

  int count[10] = {
      0,
  };

  for (char ch : mul) {
    count[ch - '0']++;
  }

  for (int i = 0; i < 10; ++i) {
    cout << count[i] << '\n';
  }

  return 0;
}