print("hello from solver!")

//    1  0
// 0  1  1
// 2  1  2

let grid = [
    [Cell(group: 1, at: Point(0, 0)), Cell(group: 1, at: Point(0, 1))],
    [Cell(group: 1, at: Point(1, 0)), Cell(group: 2, at: Point(1, 1))]
]

let constraint = GroupConstraint(vars: [
    grid[0][0], grid[0][1], grid[1][0]
])

print(constraint)
print(constraint.isSatisfied())
