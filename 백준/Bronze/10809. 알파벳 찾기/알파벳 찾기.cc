#include <iostream>

using namespace std;

int main() {
  string str;
  cin >> str;

  int count['Z' - 'A' + 1];

  for (int i = 0; i < 26; ++i) {
    count[i] = -1;
  }

  for (int i = 0; i < str.size(); ++i) {
    char ch = str[i];

    if (count[ch - 'a'] == -1) {
      count[ch - 'a'] = i;
    }
  }

  for (int i = 0; i < 26; ++i) {
    cout << count[i] << ' ';
  }

  return 0;
}