use crate::xlsdk::xlcall::{LPXLOPER12, xlerrValue};
use crate::constants;
use crate::xl::xlfunction::XLFunction;
use crate::functions::arguments;
use crate::semantics::builder::*;

use crate::xl::trigger::Trigger;
use crate::xl::caller::Caller;
use crate::xl::error::*;
use crate::xl::scalar::Scalar;


#[no_mangle]
pub extern "stdcall" fn infos_version(xl_trigger: LPXLOPER12) -> LPXLOPER12 {
    //
    // XLSDK::api_call(xlcAlert, vec![Variant::from("SDFDSFSDFSADFSDFSDSDFFSFSD"), Variant::from(3)].as_mut_slice());
    //let caller = XLRef::from_caller();
    let trigger = Trigger::from(xl_trigger);
    //s
    //let ref2 = unsafe { (*xl_ef) };
    //Logger::debug(&format!("reference {}", xlRef));
    //Logger::debug(&format!("trigger {}", trigger.valid()));

    //valid.with(&caller);

    //let caller = capture(&valid, Aux::new(Caller::try_new_scoped_by_xlfunc()));


    //XLSession::instance().add_command(caller, Box::new(Alert::new("test".to_string())));

    //let xl_ref = Variant::from(xl_ref);
    //Logger::info(&format!("reffff: {}", xl_ref));
    /*let refd = Variant::from(xl_ref);
    Logger::debug(&format!("xlref sdfsdf  {}", refd));
    let xl_ref = XLSDK::api_call(xlCoerce, vec![ refd, Variant::from(xltypeRef as i32)].as_mut_slice());
    Logger::debug(&format!("xlref sdfsdf 2 {}", xl_ref));*/

    trigger.on_validated(|| {
        let mut valid = validated::of::<Ergo<()>>();

        let caller = Caller::try_new_scoped_by_xlfunc()
            .validation_capture(&mut valid);
        Scalar::Text(String::from(constants::XLSCALA_STR_PRODUCTVERSION))
    })
    /*if (trigger.is_valid()) {
        let result =Variant::from(constants::XLSCALA_STR_PRODUCTVERSION);
        LPXLOPER12::from(result)
    } else {
        let result = Variant::from_err(xlerrValue);
        LPXLOPER12::from(result)
    }*/
}


#[no_mangle]
pub extern "stdcall" fn infos_author(xl_trigger: LPXLOPER12) -> LPXLOPER12 {
    let trigger = Trigger::from(xl_trigger);
    trigger.on_validated( || {
        Scalar::Text(String::from(constants::XLSCALA_AUTHOR))
    })
}


pub struct InfoFunctions;

impl InfoFunctions {

    #[inline(always)]
    pub fn group() -> &'static str {
        "XLScala[infos]"
    }

    pub fn version() -> XLFunction {
        builder::of::<XLFunction>()
            .group(InfoFunctions::group())
            .name("infos_version")
            .xlname("_.version")
            .args(vec![arguments::Arguments::trigger()])
            .desc("return the version number")
            .build()
    }

    pub fn author() -> XLFunction {
        builder::of::<XLFunction>()
            .group(InfoFunctions::group())
            .name("infos_author")
            .xlname("_.author")
            .args(vec![arguments::Arguments::trigger()])
            .desc("return the author name")
            .build()
    }
}