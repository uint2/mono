#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Branch<'a>(&'a str);

impl<'a> Branch<'a> {
    pub const fn new(branch: &'a str) -> Self {
        Self(branch)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }
}
