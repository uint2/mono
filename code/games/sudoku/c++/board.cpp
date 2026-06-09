#include "board.h"

#define GRAY(x) "\x1b[37m" x "\x1b[m"
const char TOP[282] = GRAY(
    "╓─────────┬─────────┬─────────╥─────────┬─────────┬─────────╥─────────┬─────────┬─────────╖");
const char MID[282] = GRAY(
    "╟─────────┼─────────┼─────────╫─────────┼─────────┼─────────╫─────────┼─────────┼─────────╢");
const char BOT[282] = GRAY(
    "╙─────────┴─────────┴─────────╨─────────┴─────────┴─────────╨─────────┴─────────┴─────────╜");
const char SEP[20] = GRAY("\x1b[37m┊\x1b[m");
const char VERT[20] = GRAY("\x1b[37m║\x1b[m");
#undef GRAY

std::ostream &operator<<(std::ostream &os, const Board &board) {
    os << TOP << std::endl;
    for (int r = 0; r < 81; r += 9) {
        if (r != 0 && r % 3 == 0) {
            os << MID << std::endl;
        }
        os << VERT;
        for (int b = 0; b < 9; b += 3) {
            os << board[r + b] << SEP << board[r + b + 1] << SEP
               << board[r + b + 2] << VERT;
        }
        os << std::endl;
    }
    return os << BOT;
}
