use crate::semantics::cause::{Cause, cause};
use crate::semantics::failed::Failed;
use crate::semantics::like::Like;
use crate::semantics::successful::Successful;


pub type Ergo<A, E> = Result<A, Cause<E>>;

impl <A, E> Successful<A> for Ergo<A, E> {
    fn successful(a: A) -> Self {
        Ok(a)
    }
}

impl <A, E> Failed<E> for Ergo<A, E> {
    fn failed(error: E) -> Self {
        Err(Cause::Terminal(error))
    }
}

impl <A, E: Clone> Like<Ergo<A, E>, Cause<E>> for Ergo<A, E> {
    fn like(&self) -> Cause<E> {
        match self {
            Ok(value) => { Cause::Mute }
            Err(c) => { c.clone() }
        }
    }
}
/*impl <A, E: Clone> Like<validated, Ergo<(), E>> for Ergo<A, E> {
    fn like(&self) -> Ergo<(), E> {
        match self {
            Ok(_) => { Ok(())}
            Err(cause) => { Err(cause.clone()) }
        }
    }
}*/

/*impl <E: Clone > Monoid<validated> for Ergo<(), E>
    where  Cause<E> :Monoid<validated> {
    fn mute() -> Self {
        Ok(())
    }
    fn is_mute(&self) -> bool {
        match self {
            Ok(()) => true,
            _ => false
        }
    }

    unsafe fn unsafe_op(left: Self::Arg, right: Self::Arg) -> Self {
        let cause = Cause::<E>::op(
            left.unwrap_err_unchecked(),
            right.unwrap_err_unchecked()
        );
        if (cause.is_mute()) {
            Ok(())
        } else {
            Err(cause)
        }
    }
}*/

/*pub trait ErgoOps {

    type ReduceForm;

    fn validation_capture(self, valid: &mut Validated<Self>) -> Self::ReduceForm;

    #[inline(always)]
    fn mute_validation_capture(self, valid: &mut Validated<Self>) -> () {
        let _ = self.validation_capture(valid);
    }

}

impl <A, E: Clone> ErgoOps for Ergo<A, E> {
    type ReduceForm = Option<A>;

    fn validation_capture(self, valid: &mut Validated<Self>) -> Self::ReduceForm {
        valid.with(&self);
        match self {
            Ok(value) => { Some(value) }
            Err(_) => { None }
        }
    }

}*/