#pragma once

/// Size of each sudoku game in bytes.
const int N = 42;

struct Game {
    unsigned char data[N];

    Game(unsigned char* buf) {}
};
