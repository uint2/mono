extension Instance: CustomStringConvertible {
    private func join_state_line(_ borders: [Character], _ state: [State]) -> String {
        var result = "│"
        for i in 0..<size {
            result.append(" \(state[i]) ")
            result.append("┼│├┤┌┬┐".contains(borders[i + 1]) ? "│" : " ")
        }
        return result
    }

    private func join_border_line(_ borders: [Character]) -> String {
        var result = ""
        for i in 0..<size {
            result.append(borders[i])
            result.append("┼├─┌┬└┴".contains(borders[i]) ? "───" : "   ")
        }
        result.append(borders.last!)
        return result
    }

    public var description: String {
        var stdout = ""
        let print = { v in stdout.append(v + "\n") }
        let margin = String(repeating: " ", count: 12)

        print("\(margin) \(colQuota.map(\.description).joined(separator: "  "))")

        for i in 0...size {
            print("\(margin) \(join_border_line(debugBorders.val[i]))")
            if i < size {
                let q = "\(rowQuota[i])".leftPadding(by: 12)
                let s = join_state_line(debugBorders.val[i], state[i])
                print("\(q) \(s)")
            }
        }
        stdout.removeLast()
        return stdout
    }
}
