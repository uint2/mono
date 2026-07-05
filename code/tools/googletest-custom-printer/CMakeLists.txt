cmake_minimum_required(VERSION 3.14)

project(nk_test)

# GoogleTest requires at least C++17
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/52eb8108c5bdec04579160ae17225d66034bd723.zip # 1.17.0
  DOWNLOAD_EXTRACT_TIMESTAMP false
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

add_library(nk_test nk_test_printer.cpp)
target_include_directories(nk_test PUBLIC .)
target_link_libraries(nk_test PUBLIC GTest::gtest_main GTest::gmock_main)

# Create compile_commands.json for clangd to latch on to.
file(CREATE_LINK ${CMAKE_BINARY_DIR}/compile_commands.json compile_commands.json SYMBOLIC)
