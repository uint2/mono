#include <cstdint>
#include <fstream>
#include <iostream>
#include <ostream>

#include "board.h"
#include "game.h"

/// Size of each sudoku game in bytes.
const int INPUT_FILE_BUF_SIZE = 42;

int main() {
    std::ifstream sudoku_data_file("../data/test.bin");

    unsigned char buf[INPUT_FILE_BUF_SIZE];

    while (true) {
        sudoku_data_file.read((char *)buf, INPUT_FILE_BUF_SIZE);
        if (sudoku_data_file.gcount() == 0) {
            break;
        }
        Game game(buf);
        Board board{};
        for (int i = 0; i < 81; i++) {
            if (i % 2 == 0) {
                board[i] = Cell::from_value(buf[i / 2] >> 4);
            } else {
                board[i] = Cell::from_value(buf[i / 2] & 0xF);
            }
        }
    }

    // OUT << IN;
    return 0;
}
