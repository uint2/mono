pub trait Checkable {
    fn is_valid(&self) -> bool;
    fn is_solved(&self) -> bool;
}
