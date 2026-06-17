#include "cpp_lib/user_age_table.h"

namespace cpp_lib {

void UserAgeTable::add(const std::string &key, uint8_t value) {

  if (std::find(this->keys.begin(), this->keys.end(), key) !=
      this->keys.end()) {
    throw KeyAlreadyInUseException();
  }
  this->keys.push_back(key);
  this->values.push_back(value);
}

std::optional<uint8_t> UserAgeTable::get(const std::string &key) const {
  auto find_iterator = std::find(this->keys.begin(), this->keys.end(), key);

  if (find_iterator == this->keys.end()) {
    return std::nullopt;
  }
  auto key_index = std::distance(this->keys.begin(), find_iterator);

  try {
    return this->values.at(key_index);
  } catch (std::out_of_range &e) {
    return std::nullopt;
  }
}

} // namespace cpp_lib
