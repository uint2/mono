struct PourPoint {
    /**
     * The more damaging fluid to pour
     */
    let fluid: State
    let waterFlow: [Point]
    let airFlow: [Point]

    var startPoint: Point { waterFlow.first! }
    var maxDamage: Int { max(waterFlow.count, airFlow.count) }

    init(at point: Point, groups: [[Int]]) {
        var flow = (water: [point], air: [point])
        let group = groups[point]
        let size = groups.count

        for row in 0..<size {
            for col in 0..<size {
                // skip points that are in different groups
                if groups[row][col] != group {
                    continue
                }

                // skip points that are the same as the first
                if row == point.row, col == point.col {
                    continue
                }

                if row >= point.row {
                    flow.water.append(Point(row, col))
                }

                if row <= point.row {
                    flow.air.append(Point(row, col))
                }
            }
        }

        self.waterFlow = flow.water
        self.airFlow = flow.air
        self.fluid = waterFlow.count > airFlow.count ? .water : .air
    }

    func getPoints(_ state: State) -> [Point] {
        switch state {
        case .water: return waterFlow
        case .air: return airFlow
        case .none: return []
        }
    }

    func getPoints() -> [Point] {
        getPoints(fluid)
    }

    func getAltPoints() -> [Point] {
        getPoints(fluid.next)
    }
}

extension PourPoint: CustomStringConvertible {
    var description: String {
        "PourPoint\(startPoint) { water: \(waterFlow), air: \(airFlow) }"
    }
}
