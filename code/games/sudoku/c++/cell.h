#pragma once

#include <bit>
#include <cstdint>
#include <iostream>
#include <ostream>

const uint16_t ALL_CANDIDATES_BITS = 0x1FF;

struct Cell {
    // Constructor.
    Cell(uint16_t bits) : bits_{bits} {};
    Cell() : bits_{0x01FF} {};

    static Cell from_value(uint8_t value);

    inline bool operator==(const Cell rhs) const { return bits_ == rhs.bits_; }

    // Bitwise operations.
    inline Cell operator&(const Cell rhs) const { return bits_ & rhs.bits_; }
    inline Cell operator|(const Cell rhs) const { return bits_ | rhs.bits_; }
    inline Cell operator&=(const Cell rhs) { return bits_ &= rhs.bits_; }
    inline Cell operator|=(const Cell rhs) { return bits_ |= rhs.bits_; }
    inline Cell operator!() const { return !bits_; }

    inline uint8_t count_ones() const { return std::popcount(bits_); }

    // Eliminates candidates that are present in the `rhs` mask.
    void eliminate(const Cell rhs) { bits_ &= !rhs.bits_; }

    // Retains candidates that are present in the `rhs` mask.
    void retain(const Cell rhs) { bits_ &= rhs.bits_; }

    friend std::ostream &operator<<(std::ostream &, const Cell &);

   private:
    uint16_t bits_;
};

const Cell ALL_CANDIDATES = Cell(ALL_CANDIDATES_BITS);
