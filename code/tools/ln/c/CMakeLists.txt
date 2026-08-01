cmake_minimum_required(VERSION 3.14)

project(ln C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_C_FLAGS "-O3")
set(CACHE CMAKE_INSTALL_PREFIX /usr/local)

message("Installing to: ${CMAKE_INSTALL_PREFIX}")

add_executable(git-ln src/main.c src/log.c)

install(TARGETS git-ln)
