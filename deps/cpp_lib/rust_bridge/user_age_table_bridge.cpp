#include "user_age_table_bridge.h"

#include <string>

namespace cpp_lib {

std::unique_ptr<UserAgeTable> new_user_age_table() {
  return std::make_unique<UserAgeTable>();
}

void add_user_age(UserAgeTable &table, rust::Str key, std::uint8_t age) {
  table.add(std::string(key), age);
}

bool get_user_age(const UserAgeTable &table, rust::Str key, std::uint8_t &age) {
  auto result = table.get(std::string(key));

  if (!result.has_value()) {
    return false;
  }

  age = result.value();
  return true;
}

} // namespace cpp_lib
