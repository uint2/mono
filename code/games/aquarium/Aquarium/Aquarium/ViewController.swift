//
//  ViewController.swift
//  Aquarium
//
//  Created by Quan Teng Foong on 18/4/23.
//

import Engine
import UIKit

class ViewController: UIViewController {
    @IBOutlet var boardTextView: UITextView!
    @IBOutlet var urlTextField: UITextField!

    var board: Board2!

    override func viewDidLoad() {
        super.viewDidLoad()
        let id = "MDo4LDM0MCw5OTA="
        if let board = try? Board(withProblemId: id) {
            print(board)
        }
    }

    @IBAction func solveButtonPressed(_: Any) {
        guard let urlString = urlTextField.text, urlString != "" else {
            // use default link
            let link = "https://aquarium2.vercel.app/api/get?id="
            let id = "MDo4LDM0MCw5OTA="
            let url = URL(string: link + id)
            let rawServerResponse = RawServerResponse.create(from: url)
            let board = Board2(from: rawServerResponse)
            self.board = board
            print(board.description)
            boardTextView.text = board.description
            return
        }
        let url = URL(string: urlString)
        let rawServerResponse = RawServerResponse.create(from: url)
        let board = Board2(from: rawServerResponse)
        self.board = board
        // engine.load(board)
        // engine.solve()
        print(board.description)
        boardTextView.text = board.description
    }
}
