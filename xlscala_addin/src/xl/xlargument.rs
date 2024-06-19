use crate::semantics::Str;
use crate::xlsdk::xlarg::Xlarg;
use typed_builder::TypedBuilder;
use crate::semantics::builder::Builder;

#[derive(TypedBuilder)]
pub struct XLArgument {
    pub(crate) kind: Xlarg,
    pub(crate) name: Str,
    pub(crate) desc: Str,
}

impl Builder for XLArgument {
    type Builder = XLArgumentBuilder;

    fn builder() -> Self::Builder {
        XLArgument::builder()
    }
}
