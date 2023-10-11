#include <iostream>

using namespace std;

int main() {
  string ans = "mixed";

  int nums[8];

  for (int i = 0; i < 8; ++i) {
    cin >> nums[i];
  }

  for (int i = 2; i < 8; ++i) {
    int curr = nums[i];
    int prev = nums[i - 1];
    int pprev = nums[i - 2];
    if (curr - prev != prev - pprev) {
      break;
    }
    if (i == 7) {
      if (curr - prev == 1) {
        ans = "ascending";
      }
      if (curr - prev == -1) {
        ans = "descending";
      }
    }
  }

  cout << ans;
}