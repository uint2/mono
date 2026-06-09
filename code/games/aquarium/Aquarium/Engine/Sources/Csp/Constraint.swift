class Constraint<T: Hashable, V: Variable<T>> {
    var vars: [V]

    init(vars: [V]) {
        self.vars = vars
    }

    func isSatisfied() -> Bool {
        fatalError("Subclasses need to implement the `isSatisfied()` method.")
    }
}

/**
 * Constraint within the same Aquarium group.
 *
 * All cells referenced in this constraint are of the same group.
 * The only thing that needs to be checked is the flow correctness.
 */
class GroupConstraint: Constraint<State, Cell> {
    let rows: [[Cell]]

    override init(vars: [Cell]) {
        let min = vars.minRow()
        var rows: [[Cell]] = Array(repeating: [], count: vars.maxRow() - min)

        for i in 0..<vars.count {
            rows[vars[i].point.row - min].append(vars[i])
        }

        self.rows = rows
        super.init(vars: vars)
    }

    override func isSatisfied() -> Bool {
        print(vars.maxRow())
        return true
    }
}
