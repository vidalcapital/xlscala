
use crate::xlsdk::xlcall::{LPXLMREF, LPXLMREF12, LPXLREF, LPXLREF12, xlcAlert, xlCoerce, xlerrValue, xlGetName, XLMREF12, XLOPER12, XLREF, XLREF12, xltypeRef};
use crate::xlsdk::variant::Variant;
use crate::xlsdk::trigger::Trigger;
use crate::xlsdk::xlcall::{LPXLOPER12, xlfCaller};
use crate::{constants, xl};
use constcat::concat;
use crate::xl::xlfunction::XLFunction;
use crate::functions::arguments;
use crate::logger::Logger;
use crate::xl::xlref::XLRef;
use crate::xl::xlsession;
use crate::xl::xlsession::XLSession;
use crate::xl::xlcommands::*;
use crate::xl::xlcommands::alert::Alert;
use crate::xlsdk::xlsdk::XLSDK;

/* Shows version string. Note that in the Excel function wizard, this shows
   as requiring one unnamed parameter. This is a longstanding Excel bug.7
 */
#[no_mangle]
pub extern "stdcall" fn xlsTime(xlTrigger: LPXLOPER12) -> LPXLOPER12 {
    //
    // XLSDK::api_call(xlcAlert, vec![Variant::from("SDFDSFSDFSADFSDFSDSDFFSFSD"), Variant::from(3)].as_mut_slice());
    //let caller = XLRef::from_caller();
    let trigger = Trigger::from(xlTrigger);
    //s
    //let ref2 = unsafe { (*xl_ef) };
    //Logger::debug(&format!("reference {}", xlRef));
    //Logger::debug(&format!("trigger {}", trigger.valid()));

    let caller = XLRef::from_caller();

    XLSession::instance().add_command(caller, Box::new(Alert::new("test".to_string())));

    //let xl_ref = Variant::from(xl_ref);
    //Logger::info(&format!("reffff: {}", xl_ref));
    /*let refd = Variant::from(xl_ref);
    Logger::debug(&format!("xlref sdfsdf  {}", refd));
    let xl_ref = XLSDK::api_call(xlCoerce, vec![ refd, Variant::from(xltypeRef as i32)].as_mut_slice());
    Logger::debug(&format!("xlref sdfsdf 2 {}", xl_ref));*/


    let result = if (trigger.valid()) {
        Variant::from(constants::XLSCALA_STR_PRODUCTVERSION)
    } else {
        Variant::from_err(xlerrValue)
    };
    LPXLOPER12::from(result)
}

pub const XLSTIME: XLFunction = XLFunction::new(
    "XLScala[tools]",
    "xlsTime",
    &[arguments::TRIGGER],
    "returns the version number of XLScala"
);