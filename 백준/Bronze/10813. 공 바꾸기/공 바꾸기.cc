#include <iostream>
#include <vector>

using namespace std;

int main() {

  int n, m;
  cin >> n >> m;
  auto nums = vector<int>(n + 1);
  for (int i = 1; i <= n; i++) {
    nums[i] = i;
  }

  for (int i = 0; i < m; i++) {
    int a, b, tmp;
    cin >> a >> b;
    tmp = nums[a];

    nums[a] = nums[b];
    nums[b] = tmp;
  }

  for (int i = 1; i <= n; i++) {
    cout << nums[i] << ' ';
  }
}