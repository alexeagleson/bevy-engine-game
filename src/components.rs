
pub struct Name(pub String);

#[derive(PartialEq)]
pub enum SeverityLevel {
    Max,
    Moderate,
    Min,
}

pub trait Severity {
    fn get_severity(&self) -> SeverityLevel;
}
