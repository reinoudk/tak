#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Increment {
    NONE,
    PATCH,
    MINOR,
    MAJOR,
}
