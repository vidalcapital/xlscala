use crate::xl::xlsession::XLSession;
use crate::functions;
use crate::functions::infos;
use crate::xl::error::Error;
use crate::xl::xlregistry::XLRegistry;
use crate::xlsdk::variant::Variant;
use crate::xlsdk::xlfn::Xlfn;
use crate::xlsdk::xlsdk::XLSDK;


#[no_mangle]
pub extern "stdcall" fn xlAutoOpen() -> i32 {

    let registry = XLRegistry::new()
        .add_function(infos::InfoFunctions::version())
        .add_function(infos::InfoFunctions::author());

    match XLSession::try_init_scoped_by_auto_open(registry) {
        Ok(()) => 1,
        Err(cause) => {
            let msg = cause.fold(& move |msg: String| move |error: &Error| {
                let ret = format!("{}\n{}", msg, error);
                ret
            })(String::from("XL Auto Open errors:\n"));
            XLSDK::api_call(Xlfn::Alert, vec![Variant::from(msg), Variant::from(2)].as_mut_slice()).unwrap();
            0
        }
    }
}
