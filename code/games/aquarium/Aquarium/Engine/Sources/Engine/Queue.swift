/**
 * Queue implemented with a singly-linked list.
 *
 * Offers O(1) adding to back and removing from front.
 */
class Queue<T> {
    class Node<T> {
        let value: T
        var next: Node?

        init(value: T, next: Node? = nil) {
            self.value = value
            self.next = next
        }

        convenience init(value: T) {
            self.init(value: value, next: nil)
        }
    }

    private var front: Node<T>?
    private var back: Node<T>?
    public var isEmpty: Bool { front == nil }

    /**
     * Adds a new value to the queue
     */
    public func enqueue(_ value: T) {
        let fresh = Node(value: value)
        if let back = back {
            back.next = fresh
        }
        back = fresh
        if front == nil {
            front = back
        }
    }

    /**
     * Retrieves the value at the front of the queue
     */
    @discardableResult
    public func dequeue() -> T? {
        defer {
            if let next = front?.next {
                front = next
            } else {
                (front, back) = (nil, nil)
            }
        }
        return front?.value
    }
}

// Display implementation for a single node
extension Queue.Node: CustomStringConvertible {
    public var description: String {
        if let next = next {
            return String(describing: next) + " <- " + "\(value)"
        }
        return "\(value)"
    }
}

// Display implementation for the queue
extension Queue: CustomStringConvertible {
    public var description: String {
        if let front = front {
            return "[" + front.description + "]"
        }
        return "[]"
    }
}
