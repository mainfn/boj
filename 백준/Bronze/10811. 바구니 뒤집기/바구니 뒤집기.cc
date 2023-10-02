#include <iostream>
#include <vector>

using namespace std;

void Swap(int& a, int& b) {
  a += b;
  b = a - b;
  a -= b;
}

int main() {
  int n, m;
  cin >> n >> m;
  vector<int> nums(n + 1);

  for (int i = 1; i <= n; ++i) {
    nums[i] = i;
  }

  for (int i = 0; i < m; ++i) {
    int from, until;
    cin >> from >> until;
    reverse(nums.begin() + from, nums.begin() + until + 1);
  }

  for (int i = 1; i <= n; ++i) {
    cout << nums[i] << ' ';
  }
}