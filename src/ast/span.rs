use std::convert::From;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
}

impl From<pest::Span<'_>> for Span {
    fn from(other: pest::Span<'_>) -> Self {
        Span {
            start: other.start(),
            end: other.end(),
        }
    }
}
