#include <iostream>
#include <set>
#include <vector>

using namespace std;

bool Comparator(const string& a, const string& b) {
  if (a.size() == b.size()) {
    return a < b;
  }
  return a.size() < b.size();
}

int main() {
  int n;
  scanf("%d\n", &n);
  set<string> words;

  for (int i = 0; i < n; i++) {
    string str;
    cin >> str;
    words.insert(str);
  }

  vector<string> words_vec(words.begin(), words.end());
  sort(words_vec.begin(), words_vec.end(), Comparator);

  for (auto w : words_vec) {
    cout << w << '\n';
  }
  return 0;
}