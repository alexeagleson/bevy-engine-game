use crate::position::Position;

pub struct Name(pub String);

pub struct Human;

pub struct Goblin;

#[derive(PartialEq)]
pub enum Severity {
    Max,
    Moderate,
    Min,
}

pub trait HasSeverity {
    fn get_severity(&self) -> Severity;
}
