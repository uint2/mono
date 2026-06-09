cmake_minimum_required (VERSION 3.14)
project(sudoku VERSION 0.0.1)
set(CMAKE_POLICY_DEFAULT_CMP0177 NEW) # quieten ryml

set(CMAKE_EXPORT_COMPILE_COMMANDS YES)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Build in Release mode by default
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()
set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
message("-- Build type: " ${CMAKE_BUILD_TYPE})

# include(FetchContent)
# FetchContent_Declare(googletest
#     GIT_REPOSITORY https://github.com/google/googletest
#     GIT_TAG b514bdc898e2951020cbdca1304b75f5950d1f59 # v1.15.2
# )
# FetchContent_MakeAvailable(googletest)

execute_process(
  COMMAND ${CMAKE_COMMAND} -E create_symlink
  ${CMAKE_BINARY_DIR}/compile_commands.json
  ${CMAKE_SOURCE_DIR}/compile_commands.json
)

add_executable(sudoku
  board.h
  cell.h
  game.h

  main.cpp
  game.cpp
  cell.cpp
  board.cpp
)
