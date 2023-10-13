#include <iostream>

using namespace std;

bool IsPalindrome(const string& str) {
  for (int i = 0; i < str.size() / 2; i++) {
    if (str[i] != str[str.size() - 1 - i]) {
      return false;
    }
  }
  return true;
}

int main() {
  while (true) {
    string input;
    cin >> input;
    if (input == "0") {
      break;
    }

    if (IsPalindrome(input)) {
      cout << "yes\n";
    } else {
      cout << "no\n";
    }
  }

  return 0;
}