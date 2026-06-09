public struct Instance {
    var state: [[State]]
    var rowQuota: [Quota]
    var colQuota: [Quota]
    let groups: Box<[[Int]]>
    let debugBorders: Box<[[Character]]>

    var size: Int { groups.val.count }

    init(rowSums: [Int], colSums: [Int], groups: [[Int]], debugBorders: Box<[[Character]]>) {
        let size = groups.count
        self.groups = Box(groups)
        self.rowQuota = rowSums.map { Quota(size: size, waterQuota: $0) }
        self.colQuota = colSums.map { Quota(size: size, waterQuota: $0) }
        self.state = groups.map { $0.map { _ in State.none }}
        self.debugBorders = debugBorders
    }

    /**
     * Tries to pour a certain fluid. If it's valid, the pour is unpourne,
     * since there is no immediate conclusion.
     *
     * If the result is invalid, we know for sure that the first fluid
     * can't be poured there, so we lock in the second fluid.
     *
     * Returns a true if changes were made
     */
    mutating func tryPour(_ fluid: State, at pourPoint: PourPoint) -> Bool {
        let delta = pour(fluid, at: pourPoint)

        if isValid() {
            unpour(fluid, from: delta)
            return false
        } else {
            unpour(fluid, from: delta)
            return !pour(fluid.next, at: pourPoint).isEmpty
        }
    }

    /**
     * Makes all forcing moves based on the current state.
     * WARNING: may lead to an invalid state. This happens when pouring
     * both air and water into a particular point leads to an invalid state
     */
    mutating func fastForward(using pourPoints: [PourPoint]) {
        var changed = true
        while changed {
            changed = false
            for pourPoint in pourPoints {
                if state[pourPoint.startPoint].isFluid {
                    continue
                }

                var delta = tryPour(.water, at: pourPoint)
                changed = changed || delta

                delta = tryPour(.air, at: pourPoint)
                changed = changed || delta

                // break off early if invalid state is already reached
                if !isValid() { return }
            }
        }
    }

    /**
     * Pours a fluid into a set of points.
     * Returns a list of affected points
     */
    @discardableResult
    mutating func pour(_ state: State, at pourPoint: PourPoint) -> [Point] {
        pour(state, into: pourPoint.getPoints(state))
    }

    /**
     * Pours a fluid into a set of points.
     * Returns a list of affected points.
     */
    @discardableResult
    mutating func pour(_ state: State, into points: [Point]) -> [Point] {
        assert(state.isFluid)

        let affected = points.filter { self.state[$0].isNone }
        affected.forEach { set(state, at: $0) }

        return affected
    }

    /**
     * Undo the changes created by pour(). Set all the points to .none
     */
    mutating func unpour(_ state: State, from points: [Point]) {
        points.forEach { unset(state, at: $0) }
    }

    /**
     * Set a point to a particular state, and update the quotas.
     *
     * unchecked precondition: state is one of water/air
     */
    private mutating func set(_ fluid: State, at point: Point) {
        assert(fluid.isFluid)

        // update the state
        state[point] = fluid

        // update the quotas
        rowQuota[point.row].decrement(fluid)
        colQuota[point.col].decrement(fluid)
    }

    /**
     * Unset a point (from water/air back to none)
     */
    private mutating func unset(_ fluid: State, at point: Point) {
        assert(fluid.isFluid)

        // update the state
        state[point] = .none

        // update the quotas
        rowQuota[point.row].increment(fluid)
        colQuota[point.col].increment(fluid)
    }
}

extension Instance: Checkable {
    func isValid() -> Bool {
        rowQuota.isValid() && colQuota.isValid()
    }

    func isSolved() -> Bool {
        rowQuota.isSolved() && colQuota.isSolved()
    }
}
