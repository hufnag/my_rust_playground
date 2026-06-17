#include "cpp_lib/template_class.h"

#include <cstdint>
#include <iostream>
#include <stdexcept>

int main(int argc, char *argv[]) {

  cpp_lib::Dict<std::string, uint8_t> name_age_dict;

  name_age_dict.add("Horst", 56);

  bool exception_caused = false;
  try {
    name_age_dict.add("Horst", 70);
  } catch (cpp_lib::KeyAlreadyInUseException) {
    std::cout << "Expected key already in use exception raised.\n";
    exception_caused = true;
  }

  if (!exception_caused) {
    throw std::runtime_error("Key already in use exception not thrown");
  }

  auto none = name_age_dict.get("Martin");
  if (none.has_value()) {
    throw std::runtime_error("Broken lookup");
  }

  name_age_dict.add("Martin", 20);
  name_age_dict.add("Jens", 36);

  auto age = name_age_dict.get("Martin");
  if (!age.has_value()) {
    throw std::runtime_error("Broken lookup");
  }

  if(age.value() != 20){
      throw std::runtime_error("Wrong value");
  }

  std::cout << "Success\n";

  return 0;
}
