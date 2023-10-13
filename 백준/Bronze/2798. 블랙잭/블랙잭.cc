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

  int closest_sum = 0;

  for (int i = 0; i < n - 2; i++) {
    for (int j = i + 1; j < n - 1; j++) {
      for (int k = j + 1; k < n; k++) {
        int sum = nums[i] + nums[j] + nums[k];
        if (sum > closest_sum && sum <= m) {
          closest_sum = sum;
        }
      }
    }
  }

  cout << closest_sum;

  return 0;
}