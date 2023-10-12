#include <iostream>

using namespace std;

int CalcScore(string line) {
  int score = 0;
  int serial = 0;

  for (char ch : line) {
    if ('X' == ch) {
      serial = 0;
    } else {
      score += ++serial;
    }
  }
  return score;
}

int main() {
  int t;
  scanf("%d\n", &t);

  for (int i = 0; i < t; ++i) {
    string line;
    getline(cin, line);
    cout << CalcScore(line) << '\n';
  }

  return 0;
}