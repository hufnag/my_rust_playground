#pragma once

#include <string>
#include <memory>
#include <optional>
#include <vector>
#include <algorithm>
#include <exception>
#include <cstdint>

namespace cpp_lib {

class KeyAlreadyInUseException : public std::exception {
public:
  const char *what() const noexcept override {
    return "The provided key is already in use";
  }
};

class UserAgeTable {
public:
  void add(const std::string &key, uint8_t value);
  std::optional<uint8_t> get(const std::string &key) const;

private:
  std::vector<std::string> keys;
  std::vector<uint8_t> values;
};

} // namespace cpp_lib
