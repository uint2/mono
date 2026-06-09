class Cell: Variable<State> {
    let point: Point
    let group: Int

    init(group: Int, at point: Point) {
        self.point = point
        self.group = group
        super.init(domain: State.all)
    }
}

extension [Cell] {
    func maxRow() -> Int {
        self.max { $0.point.row < $1.point.row }!.point.row
    }

    func minRow() -> Int {
        self.max { $0.point.row < $1.point.row }!.point.row
    }
}
