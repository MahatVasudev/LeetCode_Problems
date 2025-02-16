
#include <iostream>
#include <ostream>
#include <unordered_set>
#include <vector>
bool CheckDuplicateBrute(std::vector<int> &num) {

  for (int i = 0; i <= num.size(); i++) {
    for (int j = 0; j <= num.size(); j++) {
      if (num[i] == num[j]) {
        return true;
      }
    }
  }
  return true;
}

bool ChechDuplicateSetMethod(std::vector<int> &num) {
  return std::unordered_set<int>(num.begin(), num.end()).size() < num.size();
}

int main() {

  std::vector<int> vec = {1, 2, 3, 3};

  bool result = CheckDuplicateBrute(vec);
  bool result2 = ChechDuplicateSetMethod(vec);
  std::cout << "Result " << result;

  std::cout << "Result Set " << result2;
  return 0;
}
