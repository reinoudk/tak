#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Increment {
    None,
    Patch,
    Minor,
    Major,
}
