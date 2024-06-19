use crate::xl::range::Range;
use crate::xl::error::Ergo;

pub trait XLCommand {
    fn execute(&self, range: &Range) -> Ergo<()>;
}

