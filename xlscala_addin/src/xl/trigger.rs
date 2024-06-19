use crate::constants;
use crate::xl::caller::Caller;
use crate::xl::scalar::Scalar;
use crate::xlsdk::variant::{Variant, xltypeMask};
use crate::xlsdk::xlcall::{LPXLOPER12, xlerrDiv0, xlerrGettingData, xlerrNA, xlerrName, xlerrNull, xlerrNum, xlerrRef, xlerrValue, xloper12, xltypeBool, xltypeErr, xltypeMulti, xltypeNum, xltypeStr};

pub(crate) struct Trigger(bool);

impl Trigger {

    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.0
    }

    fn is_error(v: &xloper12) -> bool {
        match v.xltype & xltypeMask {
            xltypeErr => true,
            xlTypeMissing => true,
            xlTypeNil => true,
            xltypeBool => unsafe { v.val.xbool == 0 },
            xltypeMulti => unsafe {
                let cols = v.val.array.columns;
                let rows = v.val.array.rows;
                let size = (cols * rows) as isize;
                if (size == 0) {
                    true
                } else {
                    let p = v.val.array.lparray;
                    let mut flag = false;
                    let mut i = size;
                    while (flag == false || i != 0) {
                        flag = flag || Trigger::is_error(&(*p.offset(i)));
                        i -= 1;
                    }
                    flag
                }
            }
            _ => true
        }
    }

    #[inline(always)]
    pub fn on_validated<F: Fn(&Caller) -> Scalar>(&self, caller: Caller, f: F) -> LPXLOPER12 {
        if (self.is_valid()) {
            let scalar = f();
            let result = scalar.to_variant(caller.range);
            LPXLOPER12::from(result)
        } else {
            let result = Variant::from_err(xlerrValue);
            LPXLOPER12::from(result)
        }
    }

}

impl From<&xloper12> for Trigger {
    fn from(v: &xloper12) -> Trigger {
        Trigger (Trigger::is_error(v))
    }
}

impl From<LPXLOPER12> for Trigger {
    fn from(oper: LPXLOPER12) -> Trigger {
        let mut res = unsafe { *oper };
        Trigger::from(&res)
    }
}

impl From<&Variant> for Trigger {
    fn from(v: &Variant) -> Trigger {
        Trigger::from(&v.0)
    }
}