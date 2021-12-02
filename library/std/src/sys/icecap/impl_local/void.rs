#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

impl Void {
    pub fn void<T>(&self) -> T {
        match *self {}
    }
}
