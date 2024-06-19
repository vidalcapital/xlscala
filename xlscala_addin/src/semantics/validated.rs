use crate::semantics::coalesce::coalesce;
use crate::semantics::like::Like;
use crate::semantics::monoid::{Monoid, MonoidBox};

type Validated<A: Monoid<coalesce>> = MonoidBox<coalesce, A>;

/*pub struct validated;
pub struct Validated<A: Monoid<validated>>(pub A);

impl validated {

    #[inline(always)]
    pub fn of<A: Monoid<validated>>() -> Validated<A> {
        Validated(A::mute())
    }

    #[inline(always)]
    pub fn require<E: Clone>(flag: bool, error: E) -> Ergo<(), E> {
        if (!flag)  {
            Err(Cause::Terminal(error))
        } else {
            Ok(())
        }
    }
}

impl <A: Monoid<validated> + Clone> Validated<A> {
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        A::is_mute(&self.0)
    }

    #[inline(always)]
    pub fn unwrap(self) -> A {
        self.0
    }

    #[inline(always)]
    pub fn with<B: Like<validated, A>>(&mut self, b: &B) -> &Self {
        self.0 = A::op(self.0, b.like());
        self
    }

    /*#[inline(always)]
    fn on_valid<'a, R>(&'a self, f: impl Fn(&'a Validated<A>) -> R) -> R {
        f(self)
    }*/
}*/


/*pub trait ValidatedExtension<A: Monoid<validated>> {
    /*fn validation_capture<'a>(self, valid: &'a mut Validated<A>) -> Maybe<'a, Self::> {
        valid.with(&self);
        Maybe::new(self)
    }*/
    #[inline(always)]
    fn validation_capture(self, valid: &mut Validated<A>) -> Self {
        valid.with(&self);
        self
    }

    #[inline(always)]
    fn mute_validation_capture(self, valid: &mut Validated<A>)
        where Self: Sized {
        let _ =  self.validation_capture(valid);
    }
}

impl <A: Monoid<validated> + Clone, B: Like<validated, A>> ValidatedExtension<A> for B {
    /*fn validation_capture(self, valid: &mut Validated<A>) -> Self {
        valid.with(&self);
        self
    }*/
}*/
