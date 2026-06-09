#pragma once

#include <cstdint>
#include <ostream>

#include "cell.h"

struct Board {
    Cell cells[81];

    Cell &operator[](int i) {
        return this->cells[i];
    }

    Cell operator[](int i) const {
        return this->cells[i];
    }

    friend std::ostream &operator<<(std::ostream &os, const Board &b);
    // friend std::ostream &operator<<(std::ostream &os, const Board &b) {
    //   // writeln !(f, "{}", line::TOP) ? ;
    //   // for (j, row) in self.cells.chunks(9).enumerate() {
    //   //     if j
    //   //       != 0 && j % 3 == 0 {
    //   //         writeln !(f, "{}", line::MID) ? ;
    //   //       }
    //   //     write !(f, "{VERT}") ? ;
    //   //         for
    //   //           bx in row.chunks(3) {
    //   //             write !(f, "{}{COMMA}{}{COMMA}{}{VERT}", bx[0], bx[1],
    //   bx[2])
    //   //                 ? ;
    //   //           }
    //   //         writeln !(f) ? ;
    //   //   }
    //   // write !(f, "{} ({})", line::BOT, self.count_ones())
    // }
};
