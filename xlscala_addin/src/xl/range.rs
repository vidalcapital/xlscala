use widestring::{u32cstr, u32str};
use crate::semantics::failed::failed;
use crate::xl;
use crate::xl::error::{Cause, Ergo, Error};
use crate::xl::xlsession::XLSession;
use crate::xlsdk::variant::{Variant, xltypeMask};
use crate::xlsdk::xlcall::{LPXLOPER12, xlCoerce, xlfCaller, xloper12, xltypeRef};
pub use crate::xlsdk::xlcall::XLMREF12;
use crate::xlsdk::xlfn::Xlfn;

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) rows: u32,
    pub(crate) cols: u32,
    pub(crate) id_sheet: u32,
}

impl Range {
    #[inline(always)]
    pub fn x(&self) -> u32 { return self.x }
    #[inline(always)]
    pub fn y(&self) -> u32 { return self.y }
    #[inline(always)]
    pub fn x0(&self) -> u32 { return self.x }
    #[inline(always)]
    pub fn y0(&self) -> u32 { return self.y }
    #[inline(always)]
    pub fn x1(&self) -> u32 { return self.x + self.cols - 1 }
    #[inline(always)]
    pub fn y1(&self) -> u32 { return self.y + self.rows - 1 }

    #[inline(always)]
    pub fn cols(&self) -> u32 { return self.cols }
    #[inline(always)]
    pub fn rows(&self) -> u32 { return self.rows }

    #[inline(always)]
    pub fn id_sheet(&self) -> u32 { return self.id_sheet }

    fn contains_point(&self, x: u32, y: u32) -> bool {
        if ( (x >= self.x0()) && (x <= self.x1()) ) {
            if ( (y >= self.y0()) && (y <= self.y1()) ) {
                return true;
            }
        }
        return false;
    }

    pub fn is_contained_range(&self, other: &Range) -> bool {
        if (self.contains_point(other.x0(), other.y0())) {
            return true;
        } else if (self.contains_point(other.x1(), other.y0())) {
            return true;
        } else if (self.contains_point(other.x1(), other.y1())) {
            return true;
        } else if (self.contains_point(other.x0(), other.y1())) {
            return true;
        } else {
            if ( (self.x0() >= other.x0()) && ( (self.x0()+self.cols()-1) <= (other.x0()+other.cols()-1) ) ) {
                if ( (self.y0() >= self.y0()) && ( (self.y0()+self.rows()-1) <= (other.y0()+other.rows()-1) ) ) {
                    return true;
                }
            }
            return false;
        }
    }
}

impl TryFrom<&Variant> for Range {
    type Error = Cause;

    fn try_from(v: &Variant) -> Ergo<Range> {
        failed::with(Error::NoRange(String::from("expecting a range"))).of()
        /*match v.0.xltype & xltypeMask {
            xlTypeSRef => {
                let xl_ref = XLSession::api_call(
                    Xlfn::Coerce,
                    vec![v.clone(), Variant::from(xltypeRef as i32)].as_mut_slice()
                )?;
                let id_sheet = unsafe { xl_ref.0.val.mref.idSheet };
                let row = unsafe { (*xl_ref.0.val.mref.lpmref).reftbl[0].rwFirst };
                let rows = unsafe { (*xl_ref.0.val.mref.lpmref).reftbl[0].rwLast - row + 1 };
                let col = unsafe { (*xl_ref.0.val.mref.lpmref).reftbl[0].colFirst };
                let cols = unsafe { (*xl_ref.0.val.mref.lpmref).reftbl[0].colLast - col + 1 };
                Ok(
                    Range {
                        x: row as u32, y: col as u32,
                        rows: rows as u32, cols: cols as u32,
                        id_sheet: id_sheet as u32
                    }
                )
            }
            xltypeRef => {
                let xl_ref = v.0;
                let id_sheet = unsafe { xl_ref.val.mref.idSheet };
                let row = unsafe { (*xl_ref.val.mref.lpmref).reftbl[0].rwFirst };
                let rows = unsafe { (*xl_ref.val.mref.lpmref).reftbl[0].rwLast - row + 1 };
                let col = unsafe { (*xl_ref.val.mref.lpmref).reftbl[0].colFirst };
                let cols = unsafe { (*xl_ref.val.mref.lpmref).reftbl[0].colLast - col + 1 };
                Ok(
                    Range {
                        x: row as u32, y: col as u32,
                        rows: rows as u32, cols: cols as u32,
                        id_sheet: id_sheet as u32
                    }
                )
            }
            _ => {
                failed::with(Error::NoRange(String::from("expecting a range"))).of()
            }
        }*/
    }
}
