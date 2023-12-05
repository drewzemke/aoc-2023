use std::ops::Range;

#[derive(Debug, Clone)]
pub enum Element {
    Number(u32),
    Symbol(char),
}

#[derive(Debug)]
pub struct ElementSpan {
    pub element: Element,
    pub range: Range<usize>,
}
