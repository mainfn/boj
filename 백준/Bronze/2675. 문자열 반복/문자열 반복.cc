#include <iostream>

using namespace std;

void PrintRepeat(int &count, string &str) {
  for (int i = 0; i < str.length(); ++i) {
    for (int j = 0; j < count; ++j) {
      cout << str[i];
    }
  }
  cout << '\n';
}
int main() {

  int t;
  cin >> t;

  for (int i = 0; i < t; ++i) {
    int n;
    string str;
    cin >> n >> str;
    PrintRepeat(n, str);
  }

  return 0;
}
