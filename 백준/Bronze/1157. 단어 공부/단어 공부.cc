#include <iostream>

using namespace std;

int main() {

  string str;
  cin >> str;

  int count[26] = {
      0,
  };

  for (int i = 0; i < str.length(); ++i) {
    if (str[i] >= 'a') {
      str[i] -= 'a' - 'A';
    }
    count[str[i] - 'A']++;
  }

  int max_count = 0;
  char answer;

  for (int i = 0; i < 26; ++i) {
    if (count[i] == max_count) {
      answer = '?';
    }
    if (count[i] > max_count) {
      max_count = count[i];
      answer = i + 'A';
    }
  }

  cout << answer;

  return 0;
}