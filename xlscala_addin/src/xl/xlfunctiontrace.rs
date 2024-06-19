use crate::semantics::coalesce::coalesce;
use crate::semantics::monoid::MonoidBox;
use crate::xl::error::Cause;
use crate::xl::range::Range;
use crate::xl::scalar::XLHandle;

struct XLFunctionTrace {
    caller: Range,
    formula: String,
    validated: MonoidBox<coalesce, Cause>,
    xl_handles: Vec<XLHandle>
}


impl XLFunctionTrace {

}