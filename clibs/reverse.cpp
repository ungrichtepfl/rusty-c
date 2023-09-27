
#include <iostream>

// function definition
void _reverse(const std::string &str) {

  // store the size of the string
  size_t numOfChars = str.size();

  if (numOfChars == 1) {
    std::cout << str << std::endl;
  } else {
    std::cout << str[numOfChars - 1];

    _reverse(str.substr(0, numOfChars - 1));
  }
}

void reverse() {
  std::string str;
  std::cout << "You are now in C++ land." << std::endl;

  std::cout << "Please enter a string:" << std::endl;
  getline(std::cin, str);

  std::cout << "The reverse of " << str << " is:" << std::endl;

  _reverse(str);

  std::cout << "You are now leaving C++ land." << std::endl;
}
