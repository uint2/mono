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

extension String {
    /**
     * Add left-padding for nice printing
     */
    func leftPadding(by n: Int, char c: Character = " ") -> String {
        count < n ? String(repeatElement(c, count: n - count)) + self : String(suffix(n))
    }
}

extension URLSession {
    /**
     * Synchronously fetch HTTPS data
     */
    static func synchronousDataTask(with url: URL) -> (Data?, URLResponse?, Error?) {
        var packet: (Data?, URLResponse?, Error?)
        let sph = DispatchSemaphore(value: 0)
        let dataTask = shared.dataTask(with: url) {
            packet = ($0, $1, $2)
            sph.signal()
        }
        dataTask.resume()
        _ = sph.wait(timeout: .distantFuture)
        return packet
    }
}
