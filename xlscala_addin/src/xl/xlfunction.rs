use typed_builder::TypedBuilder;
use crate::semantics::builder::Builder;
use crate::semantics::failed::*;
use crate::semantics::Str;
use crate::semantics::successful::successful;
use crate::xl::xlargument::XLArgument;
use crate::xl::error::{Ergo, Error};
use crate::xlsdk::variant::Variant;
use crate::xlsdk::xlarg::Xlarg;
use crate::xlsdk::xlfn::Xlfn;
use crate::xlsdk::xlsdk::XLSDK;

#[derive(TypedBuilder)]
pub struct XLFunction {
    pub(crate) group: Str,
    pub(crate) name: Str,
    pub(crate) xlname: Str,
    pub(crate) args: Vec<XLArgument>,
    pub(crate) desc: Str,
    #[builder(default=true)]
    is_thread_safe: bool,
    #[builder(default=false)]
    is_async: bool,
    #[builder(default=false)]
    is_volatile: bool
}

impl XLFunction {

    pub fn register_scoped_by_auto_open(&self, xll_name: &String) -> Ergo<()> {
        let mut arg_types = String::new();
        let mut arg_names = String::new();
        for arg in self.args.iter() {
            arg_types.push_str(arg.kind.to_code());
            arg_names.push_str(arg.name);
            arg_names.push_str(",");
        }
        arg_types.push_str(Xlarg::LP_OPER12.to_code());
        if self.is_thread_safe {
            arg_types.push_str("$");
        } else {
            arg_types.push_str("#");
        }
        if self.is_volatile {
            arg_types.push_str("!");
        }
        let mut opers = vec![
            Variant::from(xll_name.clone()),
            Variant::from(self.name),
            Variant::from(arg_types),
            Variant::from(self.xlname),
            Variant::from(arg_names),
            Variant::from(1),
            Variant::from(self.group),
            Variant::missing(),
            Variant::missing(),
            Variant::from(self.desc),
        ];
        for arg in self.args.iter() {
            opers.push(Variant::from(arg.desc));
        }
        match XLSDK::api_call(Xlfn::Register, opers.as_mut_slice()) {
            Ok(result) => successful::of(),
            Err(error)=> {
                let msg = format!("error registering function {} with error: {}", self.name, error);
                failed::with(Error::Registration(msg)).of()
            }
        }
    }

}

impl Builder for XLFunction {
    type Builder = XLFunctionBuilder;

    fn builder() -> Self::Builder {
        XLFunction::builder()
    }
}