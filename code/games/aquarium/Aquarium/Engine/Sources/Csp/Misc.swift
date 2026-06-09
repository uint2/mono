import Foundation

/**
 * The three possible states of each Cell
 */
enum State: Character, CustomStringConvertible, Equatable {
    case none = " "
    case air = "×"
    case water = "■"

    var isNone: Bool { self == .none }

    var isFluid: Bool { self == .water || self == .air }

    static var all: [Self] { [.none, .air, .water] }

    var next: Self {
        switch self {
        case .none: return .none
        case .water: return .air
        case .air: return .water
        }
    }

    public var description: String { String(rawValue) }
}

/**
 * Box class to pass things around by reference
 */
class Box<T> {
    var val: T
    init(_ val: T) {
        self.val = val
    }
}

protocol Checkable {
    func isValid() -> Bool
    func isSolved() -> Bool
}

struct Point: CustomStringConvertible {
    let row: Int
    let col: Int

    init(_ row: Int, _ col: Int) {
        self.row = row
        self.col = col
    }

    var description: String { "(\(row), \(col))" }
}

extension [[Int]] {
    subscript(p: Point) -> Int {
        get { self[p.row][p.col] } set(v) { self[p.row][p.col] = v }
    }
}

extension [[State]] {
    subscript(p: Point) -> State {
        get { self[p.row][p.col] } set(v) { self[p.row][p.col] = v }
    }
}
