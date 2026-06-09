struct Quota: Checkable, CustomStringConvertible {
    var water: Int
    var air: Int

    init(size: Int, waterQuota: Int) {
        self.water = waterQuota
        self.air = size - waterQuota
    }

    mutating func increment(_ state: State) {
        switch state {
        case .water: water += 1
        case .air: air += 1
        default: ()
        }
    }

    mutating func decrement(_ state: State) {
        switch state {
        case .water: water -= 1
        case .air: air -= 1
        default: ()
        }
    }

    func isValid() -> Bool {
        water >= 0 && air >= 0
    }

    func isSolved() -> Bool {
        water == 0 && air == 0
    }

    var description: String { "\(water),\(air)" }
}

extension [Quota]: Checkable {
    func isValid() -> Bool {
        allSatisfy { q in q.isValid() }
    }

    func isSolved() -> Bool {
        allSatisfy { q in q.isSolved() }
    }
}
