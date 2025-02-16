

#include <iostream>
#include <unordered_map>
#include <vector>
std::vector<int> TwoSum(std::vector<int> &map, int target) {

  int n = map.size();

  std::unordered_map<int, int> mapin;

  for (int i = 0; i < n; i++) {
    int subst = target - map.at(i);

    if (mapin.find(subst) != mapin.end()) {
      return {mapin[subst], i};
    }

    mapin.insert({map.at(i), i});
  }

  return {-1, -1};
}

int main() {

  std::vector<int> map = {1, 4, 5, 3};

  std::vector<int> value = TwoSum(map, 8);

  std::cout << "[" << value[0] << "," << value[1] << "]";

  return 0;
}
