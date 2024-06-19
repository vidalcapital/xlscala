use crate::xlsdk::error::{Error, ErrorKind, Result};
use crate::xlsdk::entrypoint::{excel12v, excel12v_0};
use crate::xlsdk::variant::Variant;
use crate::xlsdk::xlcall::{LPXLOPER12, xlretSuccess};
use crate::xlsdk::xlfn::Xlfn;

pub struct XLSDK;

impl XLSDK {
    pub(crate) fn api_call(xlfn: Xlfn, opers: &mut [Variant]) -> Result<Variant> {
        let mut args: Vec<LPXLOPER12> = Vec::with_capacity(opers.len());
        for oper in opers.iter_mut() {
            args.push(oper.as_mut_xloper());
        }
        let mut result = Variant::default();
        let res = excel12v(i32::from(xlfn), result.as_mut_xloper(), &args);
        if (res == xlretSuccess as i32) {
            Ok(result)
        } else {
            Err(
                Error {
                    xlfn: xlfn,
                    kind:  ErrorKind::from(res as u32)
                }
            )
        }
    }

    pub fn api_call_no_arg(xlfn: Xlfn) -> Result<Variant> {
        let mut result = Variant::default();
        let res = excel12v_0(i32::from(xlfn), result.as_mut_xloper());
        if (res == xlretSuccess as i32) {
            Ok(result)
        } else {
            Err(
                Error {
                    xlfn: xlfn,
                    kind:  ErrorKind::from(res as u32)
                }
            )
        }
    }
}