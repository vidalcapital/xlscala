use thiserror::Error;
use crate::xlsdk::xlfn::Xlfn;
use crate::semantics::successful::*;
use crate::semantics::failed::*;
use crate::semantics::like::Like;


#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("range overflow")]
    RangeOverflow,
    #[error("invalid object xl handle: {0}")]
    InvalidXLHandle(String),
    #[error("maximum number of object reached")]
    ObjectNumberOverflow,
    #[error("{0}")]
    ObjectHandler(String),
    #[error("{0}")]
    Conversion(String),
    #[error("{0}")]
    Cell(String),
    #[error("{0}")]
    NoRange(String),
    #[error("{0}")]
    Registration(String),
    /*#[error("{0}")]
    Failed(String),*/
    #[error("api call failled for {xlfn:?}, error: {kind:?}")]
    ApiCall {
        xlfn: Xlfn,
        kind: crate::xlsdk::error::ErrorKind
    },
}

pub type Ergo<A> = crate::semantics::ergo::Ergo<A, Error>;
pub type Cause = crate::semantics::cause::Cause<Error>;


impl From<crate::xlsdk::error::Error> for Cause {
    fn from(value: crate::xlsdk::error::Error) -> Self {
        Cause::Terminal(
            Error::ApiCall {
                xlfn: value.xlfn,
                kind: value.kind
            }
        )
    }
}

impl From<Error> for Cause {
    fn from(value: Error) -> Self {
        Cause::Terminal(value)
    }
}