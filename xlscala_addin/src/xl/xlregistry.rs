use std::vec::Vec;
use crate::semantics::cause::cause;
use crate::semantics::coalesce::{coalesce, Coalesce};
use crate::semantics::validated::*;
use crate::xl::error::*;
use crate::xl::xlfunction::XLFunction;

pub struct XLRegistry{
    pub(crate) functions: Vec<XLFunction>,
}

impl XLRegistry {

    pub fn new() -> XLRegistry {
        XLRegistry {
            functions: Vec::new()
        }
    }

    pub fn add_function(mut self, xl_func: XLFunction) -> Self {
        self.functions.push(xl_func);
        self
    }


    pub fn register_scoped_by_auto_open(&self, xll_name: &String) -> Ergo<()> {
        let mut valid = coalesce::of::<Cause>();
        for xlf in self.functions.iter() {
            xlf.register_scoped_by_auto_open(xll_name)
                .mute_coalesce(& mut valid);
        }
        match valid.unwrap() {
            Cause::Mute => { Ok(())}
            c => { Err(c) }
        }
    }

}