use std::fmt::{Display, Formatter};
use crate::xlsdk::xlcall::{xlcAlert, xlcCalculation, xlcEcho, xlcMessage, xlCoerce, xlcOnRecalc, xlfCaller, xlfGetCell, xlfGetDocument, xlfGetName, xlfRegister, xlfUnregister, xlGetName};


#[derive(Debug, Clone, Copy)]
pub enum Xlfn {
    XllName,
    Register,
    Unregister,
    OnRecalc,
    Alert,
    Coerce,
    GetName,
    GetCell,
    Calculation,
    GetDocument,
    Echo, Caller, Message
}

impl Display for Xlfn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Xlfn::XllName =>  write!(f, "xlGetName"),
            Xlfn::GetName =>  write!(f, "xlfGetName"),
            Xlfn::Register =>  write!(f, "xlfRegister"),
            Xlfn::Unregister =>  write!(f, "xlfUnregister"),
            Xlfn::OnRecalc => write!(f, "xlcOnRecalc"),
            Xlfn::Coerce=>  write!(f, "xlCoerce"),
            Xlfn::Alert => write!(f, "xlcAlert"),
            Xlfn::GetCell => write!(f, "xlfGetCell"),
            Xlfn::Calculation => write!(f, "xlcCalculation"),
            Xlfn::GetDocument => write!(f, "xlfGetDocument"),
            Xlfn::Echo => write!(f, "xlcEcho"),
            Xlfn::Caller => write!(f, "xlfCaller"),
            Xlfn::Message => write!(f, "xlcMessage"),
        }
    }
}

impl From<Xlfn> for i32 {
    fn from(value: Xlfn) -> Self {
        match value {
            Xlfn::XllName => xlGetName as i32,
            Xlfn::GetName => xlfGetName as i32,
            Xlfn::Register => xlfRegister as i32,
            Xlfn::Unregister => xlfUnregister as i32,
            Xlfn::OnRecalc => xlcOnRecalc as i32,
            Xlfn::Alert => xlcAlert as i32,
            Xlfn::Coerce => xlCoerce as i32,
            Xlfn::GetCell => xlfGetCell as i32,
            Xlfn::Calculation => xlcCalculation as i32,
            Xlfn::GetDocument => xlfGetDocument as i32,
            Xlfn::Echo => xlcEcho as i32,
            Xlfn::Caller => xlfCaller as i32,
            Xlfn::Message => xlcMessage as i32,
        }
    }
}