use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::xlsdk::xlcall;
use crate::xlsdk::xlfn::Xlfn;

#[derive(Error, Debug, Clone, Copy)]
pub enum ErrorKind {
    #[error("the command or function was terminated abnormally (internal abort)")]
    Abort,
    #[error("an invalid function number was supplied")]
    InvXlfn,
    #[error("an invalid number of arguments was entered")]
    InvCount,
    #[error("an invalid XLOPER or XLOPER12 was passed to the function, or an argument of the wrong type was used")]
    InvXloper,
    #[error("a stack overflow occurred")]
    StackOvfl,
    #[error("a command-equivalent function failed")]
    Failed,
    #[error("an attempt was made to dereference a cell that has not been calculated yet, because it is scheduled to be recalculated after the current cell")]
    Uncalced,
    #[error(" n attempt was made to call a function that is not, or might not be, thread safe during a multithreaded recalculation of the workbook")]
    NotThreadSafe,
    #[error("the asynchronous function handle is invalid")]
    InvAsynchronousContext,
    #[error("the call is not supported on clusters")]
    NotClusterSafe,
    #[error("unknown error return code")]
    Unknown,
}
#[derive(Error, Debug, Clone, Copy)]
pub(crate) struct Error {
    pub(crate) xlfn: Xlfn,
    pub(crate) kind: ErrorKind
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "XLSDK api error calling {} with detail: {}", self.xlfn, self.kind)
    }
}


impl From<u32> for ErrorKind {
    fn from(value: u32) -> Self {
        match value  {
            xlcall::xlretAbort =>  Self::Abort,
            xlcall::xlretInvXlfn =>  Self::InvXlfn,
            xlcall::xlretInvCount =>  Self::InvCount,
            xlcall::xlretInvXloper => Self::InvXloper,
            xlcall::xlretStackOvfl =>  Self::StackOvfl,
            xlcall::xlretFailed => Self::Failed,
            xlcall::xlretUncalced => Self::Uncalced,
            xlcall::xlretNotThreadSafe =>  Self::NotThreadSafe,
            xlcall::xlretInvAsynchronousContext =>  Self::InvAsynchronousContext,
            xlcall::xlretNotClusterSafe=> Self::NotClusterSafe,
            _ => Self::Unknown
        }
    }

}

pub type Result<A> = std::result::Result<A, Error>;