use crate::xl::xlsession::XLSession;

#[no_mangle]
pub extern "stdcall" fn xlOnRecalc() -> bool {
    XLSession::on_recalc();
    true
}

