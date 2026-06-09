import Engine

class Runner {
    enum Size: Int { case six = 6; case ten = 10; case fifteen = 15 }
    enum Difficulty: String { case easy; case normal; case hard }

    let size: Size
    let difficulty: Difficulty
    let id: Int

    init(_ size: Size, _ difficulty: Difficulty, _ id: Int) {
        self.size = size
        self.difficulty = difficulty
        self.id = id
    }

    /**
     * Reaches into a relative directory to read a pre-pulled game of aquarium
     */
    func filename() -> String {
        let (s, d) = (size.rawValue, difficulty.rawValue)
        return "../../problems/problem-db/\(s)x\(s)_\(d)_v\(id).json"
    }

    func run() throws {
        var game = try! Game(withJsonFile: filename())

        // print("START STATE:")
        // print(game.makeInstance())

        print("SOLVED STATE:")
        print(try! game.solve(), game.backtrackCounter)
    }

    static func all() -> [Runner] {
        let sizes: [Size] = [.six, .ten, .fifteen]
        let difficulties: [Difficulty] = [.easy, .normal, .hard]
        var runners = [Runner]()

        for s in sizes {
            for d in difficulties {
                for id in 1...6 {
                    runners.append(Runner(s, d, id))
                }
            }
        }

        return runners
    }
}

print("--- START EngineCli ---\n")

for runner in Runner.all() {
    try runner.run()
}

print("\n--- END EngineCli ---")
