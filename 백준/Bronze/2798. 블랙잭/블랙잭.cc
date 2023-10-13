#include <iostream>
#include <vector>

using namespace std;

int main() {
  int n, m;
  cin >> n >> m;

  vector<int> nums(n);

  for (int i = 0; i < n; i++) {
    cin >> nums[i];
  }
  sort(nums.begin(), nums.end());

  int max_sum = 0;
  for (int i = 0; i < nums.size(); i++) {
    for (int j = 0; j < nums.size(); j++) {
      if (j == i)
        continue;
      for (int k = 0; k < nums.size(); k++) {
        if (k == i || k == j)
          continue;

        int sum = nums[i] + nums[j] + nums[k];
        if (sum > max_sum && sum <= m) {
          max_sum = sum;
        }
      }
    }
  }

  cout << max_sum;

  return 0;
}