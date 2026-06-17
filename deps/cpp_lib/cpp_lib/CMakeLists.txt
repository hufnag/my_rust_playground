cmake_minimum_required(VERSION 3.20)

project(
  cpp_lib
  VERSION 0.1.0
  LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS On)

include_directories(include)

add_library(user_age_table user_age_table.cpp)

add_executable(main main.cpp)
target_link_libraries(main PRIVATE cpp_lib)
