
#include <iostream>
#include <string>
#include <unordered_map>
bool isAnagram(std::string s, std::string t) {

  std::unordered_map<char, int> countS, countT;

  for (int i = 0; i <= s.length(); i++) {
    countS[s[i]]++;
    countT[t[i]]++;
  }

  return countS == countT;
}

int main() {

  std::string s = "acvsa", t = "avcas";

  bool result = isAnagram(s, t);

  std::cout << result;
  return 0;
}
