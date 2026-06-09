/// can represent one cell in an aquarium board
/// T in this case will be the state of the cell.
class Variable<T: Hashable> {
    var domain: Set<T>

    init(domain: any Sequence<T>) {
        self.domain = Set(domain)
    }

    var domainSize: Int { domain.count }
}
