#pragma once

#include <iterator>
#include <optional>
#include <vector>

namespace cpp_lib {

class KeyAlreadyInUseException : public std::exception {
public:
  const char *what() const noexcept override {
    return "The provided key is already in use";
  }
};

template <typename K, typename V> class Dict {
public:
  void add(K key, V value) {

    if (std::find(this->keys.begin(), this->keys.end(), key) !=
        this->keys.end()) {
      throw KeyAlreadyInUseException();
    }
    this->keys.push_back(key);
    this->values.push_back(value);
  }

  std::optional<V> get(K key) {
    auto find_iterator = std::find(this->keys.begin(), this->keys.end(), key);

    if (find_iterator == this->keys.end()) {
      return std::nullopt;
    }
    auto key_index = std::distance(this->keys.begin(), find_iterator);

    try {
      return this->values.at(key_index);
    } catch (std::out_of_range e) {
      return std::nullopt;
    }
  }

private:
  std::vector<K> keys;
  std::vector<V> values;
};
} // namespace cpp_lib
