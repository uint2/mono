public struct Game {
    let groups: [[Int]]
    let colSums: [Int]
    let rowSums: [Int]
    let debugBorders: [[Character]]

    public var backtrackCounter = 0

    /**
     * A list of points that pouring can start from, where each point
     * produces a unique result after the fluid flows.
     *
     * Used for making forcing moves. Sorted by most damaging first
     */
    var pourPoints: [PourPoint]

    var size: Int { groups.count }

    public init(colSums: [Int], rowSums: [Int], groups: [[Int]]) {
        self.colSums = colSums
        self.rowSums = rowSums
        self.groups = groups
        self.pourPoints = Game.getPourPoints(groups: groups)

        // generate borders to debug with
        let it = (0...groups.count)
        self.debugBorders = it.map { r in it.map { c in Game.border(groups, r, c) }}
    }

    /**
     * Get a list of pouring points from a group matrix.size
     */
    private static func getPourPoints(groups: [[Int]]) -> [PourPoint] {
        let size = groups.count
        var points = [Point]()
        for r in 0..<size {
            for c in 0..<size {
                let g = groups[r][c]
                if !points.contains(where: { p in p.row == r && groups[p] == g }) {
                    points.append(Point(r, c))
                }
            }
        }
        return points
            .map { PourPoint(at: $0, groups: groups) }
            .sorted { $0.maxDamage > $1.maxDamage }
    }

    /**
     * Create a new Instance from the Game's initial state.
     * Useful for printing a game board from the outside.
     *
     * ```
     * print(game.makeInstance())
     * ```
     */
    public func makeInstance() -> Instance {
        Instance(
            rowSums: rowSums,
            colSums: colSums,
            groups: groups,
            debugBorders: Box(debugBorders))
    }

    /**
     * Recursive solver using a backtracking method.
     * 1. Apply all forcing moves to the current state
     * 2. DFS on all next possible moves
     */
    private mutating func backtrack(_ prev: Instance) -> Instance? {
        backtrackCounter += 1
        if !prev.isValid() { return nil }

        // Insanely (but conveniently in this case), `inst` is a deep
        // copy of `prev` but its groups remain as a reference to
        // `self.groups`. Insane because this is a result of mere
        // variable assignment.
        var inst = prev

        // Make all forcing moves on the cloned instance
        inst.fastForward(using: pourPoints)

        if !inst.isValid() {
            return nil
        }

        if inst.isSolved() {
            return inst
        }

        // get a list of possible actions to take from this juncture.
        //
        // filter out those actions whose starting point (the point
        // at which the fluid is poured) is already occupied.
        let pours = pourPoints.filter { inst.state[$0.startPoint].isNone }

        for pour in pours {
            let delta = inst.pour(pour.fluid, into: pour.getPoints())
            if let result = backtrack(inst) {
                return result
            } else {
                inst.unpour(pour.fluid, from: delta)
                // Reaching here means the entire subtree of
                // backtracking into pouring the first fluid fails.
                //
                // Hence we are forced to pour the other fluid
                inst.pour(pour.fluid.next, into: pour.getAltPoints())

                // and of course, if we are forced into an invalid state,
                // break out of the loop
                if !inst.isValid() {
                    return nil
                }
            }
        }

        return nil
    }

    /**
     * Public API to solve a game.
     */
    public mutating func solve() throws -> Instance {
        var inst = makeInstance()

        // the first big pass
        inst.fastForward(using: pourPoints)
        pourPoints = pourPoints.filter { p in inst.state[p.startPoint].isNone }

        if let result = backtrack(inst) {
            inst = result
        }

        if !inst.isSolved() {
            throw GameError.unsolved
        }

        var pretty = makeInstance()
        pretty.state = inst.state

        return pretty
    }

    private enum GameError: Error {
        case unsolved
    }
}
