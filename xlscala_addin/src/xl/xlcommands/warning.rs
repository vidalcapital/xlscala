use crate::semantics::successful::*;
use crate::xl::range::Range;
use crate::xl::error::*;
use crate::xl::xlcommand::XLCommand;
use crate::xl::xlsession::XLSession;
use crate::xlsdk::variant::Variant;
use crate::xlsdk::xlfn::Xlfn;

pub struct Warning(String);


impl XLCommand for Warning {
    fn execute(&self, xlrange: &Range) -> Ergo<()> {
        XLSession::api_call(Xlfn::Alert, vec![Variant::from(self.0.clone()), Variant::from(2)].as_mut_slice())?;
        successful::of()
    }
}
