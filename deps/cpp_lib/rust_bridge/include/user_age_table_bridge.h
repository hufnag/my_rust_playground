#pragma once

#include "cpp_lib/user_age_table.h"
#include "rust/cxx.h"

#include <cstdint>
#include <memory>

namespace cpp_lib {

std::unique_ptr<UserAgeTable> new_user_age_table();
void add_user_age(UserAgeTable &table, rust::Str key, std::uint8_t age);
bool get_user_age(const UserAgeTable &table, rust::Str key, std::uint8_t &age);

} // namespace cpp_lib
