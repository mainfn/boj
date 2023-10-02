#include <stdio.h>
#include <iostream>
#include <vector>

using namespace std;

int main() {
  int n;
  int max_score = -999999;
  cin >> n;

  vector<int> scores(n);
  double score_sum = 0.0;

  for (int i = 0; i < n; ++i) {
    int score;
    cin >> score;
    max_score = max(score, max_score);
    scores[i] = score;
  }

  for (int i = 0; i < n; ++i) {
    score_sum += (double)scores[i] / max_score * 100;
  }

  printf("%.2f", score_sum / n);
}