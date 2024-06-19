use crate::semantics::successful::successful;
use crate::xl::error::Ergo;
use crate::xl::range::Range;
use crate::xl::xlsession::XLSession;
use crate::xlsdk::variant::Variant;
use crate::xlsdk::xlcall::xlcFormula;
use crate::xlsdk::xlfn::Xlfn;
use crate::xlsdk::xlsdk::XLSDK;

pub struct Caller {
    pub(crate) range: Range,
    formula: String
}

impl Caller {

    pub fn try_new_scoped_by_xlfunc() -> Ergo<Caller> {
        //XLSDK::api_call(Xlfn::Message, vec![Variant::from(1), Variant::from("+")].as_mut_slice());
        let xl_caller = XLSession::api_call_no_arg(Xlfn::Caller)?;
        //XLSDK::api_call(Xlfn::Message, vec![Variant::from(1), Variant::from("+")].as_mut_slice());
        let range= Range::try_from(&xl_caller)?;
        //XLSDK::api_call(Xlfn::Message, vec![Variant::from(1), Variant::from("+")].as_mut_slice());
        //let XLSession::api_call_no_arg(Xlfn::GetCell)
        let caller = Caller {
            range: range,
            formula: String::from("FDSF")
        };
        successful::with(caller).of()
    }
}
