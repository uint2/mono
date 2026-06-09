#include "cell.h"

#include <bit>

Cell Cell::from_value(uint8_t value) {
    switch (value) {
        case 0:
            return Cell(ALL_CANDIDATES_BITS);
        case 1:
            return Cell(0b000000001);
        case 2:
            return Cell(0b000000010);
        case 3:
            return Cell(0b000000100);
        case 4:
            return Cell(0b000001000);
        case 5:
            return Cell(0b000010000);
        case 6:
            return Cell(0b000100000);
        case 7:
            return Cell(0b001000000);
        case 8:
            return Cell(0b010000000);
        case 9:
            return Cell(0b100000000);
        default:
            std::cout << "Invalid cell input: " << (unsigned int)value
                      << std::endl;
            std::exit(1);
    }
    return Cell(0);
}

std::ostream &operator<<(std::ostream &os, const Cell &cell) {
    if (cell.count_ones() == 1) {
        int value = 0;
        switch (cell.bits_) {
            case 0b000000001:
                value = 1;
                break;
            case 0b000000010:
                value = 2;
                break;
            case 0b000000100:
                value = 3;
                break;
            case 0b000001000:
                value = 4;
                break;
            case 0b000010000:
                value = 5;
                break;
            case 0b000100000:
                value = 6;
                break;
            case 0b001000000:
                value = 7;
                break;
            case 0b010000000:
                value = 8;
                break;
            case 0b100000000:
                value = 9;
        }
        return os << "    \x1b[32m" << value << "\x1b[m    ";
    }
    for (short i = 1; i <= 9; ++i) {
        if ((cell & 1) == 1) {
            os << "\x1b[33m" << i << "\x1b[m";
        } else {
            os << "\x1b[37m.\x1b[m";
        }
    }
    return os;
}
