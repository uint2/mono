import Foundation

/**
 * Convenience routines for initializing a Game.
 */
extension Game {
    /**
     * Response structure from `https://aquarium2.vercel.app/api/get`
     */
    private struct JSONBoard: Codable {
        struct Sums: Codable { let cols: [Int]; let rows: [Int] }
        let id: String
        let size: Int
        let sums: Sums
        let matrix: [[Int]]
    }

    /**
     * Initialize using a string that is JSON.
     */
    public init(withJson json: String) throws {
        let decoder = JSONDecoder()
        let j = try decoder.decode(JSONBoard.self, from: json.data(using: .utf8)!)
        self.init(colSums: j.sums.cols, rowSums: j.sums.rows, groups: j.matrix)
    }

    /**
     * Initialize by reading a file.
     */
    public init(withJsonFile path: String) throws {
        let cwd = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
        let fileUrl = cwd.appendingPathComponent(path)
        let json = try String(contentsOf: fileUrl, encoding: .utf8)
        try self.init(withJson: json)
    }

    /**
     * Pulls a board from the Aquarium website, and initializes it.
     */
    public init(withProblemId: String) throws {
        let url = URL(string: "https://aquarium2.vercel.app/api/get?id=" + withProblemId)!
        let (raw, _, _) = URLSession.synchronousDataTask(with: url)
        try self.init(withJson: String(data: raw!, encoding: .utf8)!)
    }
}

/**
 * Pre-processing for border printing
 */
extension Game {
    /** Get the surrounding groups of a border point.
     *  Bounds belong to group 0.
     *  [<upper-left>, <upper-right>, <lower-left>, <lower-right>]
     */
    static func surrounding_groups(_ groups: [[Int]], _ r: Int, _ c: Int) -> (Int, Int, Int, Int) {
        let (n, g) = (groups.count, groups)
        //       ↖︎  ↗︎  ↙︎  ↘︎
        var t = (0, 0, 0, 0)
        if r > 0 {
            if c > 0 { t.0 = g[r - 1][c - 1] } // ↖︎
            if c < n { t.1 = g[r - 1][c] } // ↗︎
        }
        if r < n {
            if c > 0 { t.2 = g[r][c - 1] } // ↙︎
            if c < n { t.3 = g[r][c] } // ↘︎
        }
        return t
    }

    static func border(_ groups: [[Int]], _ r: Int, _ c: Int) -> Character {
        let (ul, ur, dl, dr) = Game.surrounding_groups(groups, r, c)
        let (L, R, U, D) = (ul == dl, ur == dr, ul == ur, dl == dr)
        return U && D ? ur == dl ? " " : "─" : U ? L ? "┌" : R ? "┐" : "┬" :
            D ? L ? "└" : R ? "┘" : "┴" : L && R ? "│" : L ? "├" : R ? "┤" : "┼"
    }
}
