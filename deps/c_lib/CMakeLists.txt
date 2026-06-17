cmake_minimum_required(VERSION 3.20)

set(CMAKE_EXPORT_COMPILE_COMMANDS On)

project(
  c_lib
  VERSION 0.1.0
  LANGUAGES C)

add_library(c_lib src/c_lib.c)

add_executable(calculator src/calculator.c)
target_link_libraries(calculator PRIVATE c_lib)

include_directories(include)
