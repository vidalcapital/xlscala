use crate::semantics::builder::builder;
use crate::xl::xlargument::XLArgument;
use crate::xlsdk::xlarg::Xlarg;


pub struct Arguments;

impl Arguments {
    pub fn trigger() -> XLArgument {
        builder::of::<XLArgument>()
            .kind(Xlarg::LP_OPER12)
            .name("trigger")
            .desc("trigger")
            .build()
    }


    pub fn cellref() -> XLArgument {
        builder::of::<XLArgument>()
            .kind(Xlarg::LP_REF12)
            .name("cell reference")
            .desc("cell reference")
            .build()
    }
}