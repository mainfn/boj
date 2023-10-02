#include <iostream>

using namespace std;

int main() {
  int nums[1001] = {
      0,
  };

  for (int i = 0; i < 10; ++i) {
    int num;
    cin >> num;
    nums[num % 42] = 1;
  }

  int total_count = 0;
  for (int i = 0; i < 1001; ++i) {
    bool is_exists = nums[i] == 1;
    if (is_exists) {
      total_count++;
    }
  }
  cout << total_count;

  return 0;
}