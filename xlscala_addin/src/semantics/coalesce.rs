use crate::semantics::cause::{Cause, cause};
use crate::semantics::ergo::Ergo;
use crate::semantics::monoid::{Monoid, MonoidBox};

pub trait Coalesce<Op> {
    type Coalesced: Monoid<Op>;
    type Residual;

    fn coalesce(self, mbox: &mut MonoidBox<Op, Self::Coalesced>) -> Self::Residual;

    #[inline(always)]
    fn mute_coalesce(self, mbox: &mut MonoidBox<Op, Self::Coalesced>) -> () {
        let _ = self.coalesce(mbox);
    }
}

pub struct coalesce;

impl <A, E> Coalesce<coalesce> for Ergo<A, E> {
    type Coalesced = Cause<E>;
    type Residual = Option<A>;

    fn coalesce(self, mbox: &mut MonoidBox<coalesce, Self::Coalesced>) -> Self::Residual {
        match self {
            Ok(value) => {
                Some(value)
            }
            Err(c) => {
                unsafe {
                    mbox.value = Cause::op(*mbox.value, c);
                };
                None
            }
        }
    }
}

impl coalesce {
    pub fn of<A: Monoid<coalesce>>() -> MonoidBox<coalesce, A> {
        MonoidBox::new(A::mute())
    }
}