use crate::semantics::coalesce::coalesce;
use crate::semantics::cons::Cons;
use crate::semantics::ergo::Ergo;
use crate::semantics::failed::Failed;
use crate::semantics::monoid::Monoid;
use crate::semantics::mute::Mute;

#[derive(Clone)]
pub enum Cause<A> {
    Mute,
    Terminal(A),
    Cons(Box<(Cause<A>, Cause<A>)>)
}

impl <A> Cause<A> {

    /*#[inline(always)]
    pub fn to_ergo(self) -> Ergo<(), A> {
        match self {
            None => { Ok(()) }
            _ => { Err(self) }
        }
    }*/

    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        match self {
            Cause::Mute => { true }
            _ => { false }
        }
    }

    #[inline(always)]
    pub fn fold<'a, R, F: FnOnce(&A) -> R>(&'a self, f: &'a impl Fn(R) -> F) ->  impl FnOnce(R) -> R + 'a {
        move |r0: R| {
            match &self {
                Cause::Mute => { r0 }
                Cause::Terminal(a) => { f(r0)(a) }
                Cause::Cons(cons) => { cons.0.fold(f)(cons.1.fold(f)(r0)) }
            }
        }
    }

}

impl <A> Monoid<coalesce> for Cause<A> {
    fn mute() -> Self {
        Cause::Mute
    }

    fn is_mute(&self) -> bool {
        match self {
            Cause::Mute => true,
            _ => false
        }
    }
    unsafe fn unsafe_op(left: Self, right: Self) -> Self {
        Cause::Cons(Box::new((left, right)))
    }
}

impl <A> Cons<(Cause<A>, Cause<A>)> for Cause<A>
    where Self: Monoid<coalesce> {
    fn construct(arg: (Cause<A>, Cause<A>)) -> Self {
        Self::op(arg.0, arg.1)
    }
}

impl <A> Failed<A> for Cause<A> {
    fn failed(error: A) -> Self {
        Cause::Terminal(error)
    }
}

impl <A> Mute for Cause<A>
    where Self: Monoid<coalesce> {
    fn mute() -> Self {
        <Cause<A> as Monoid<coalesce>>::mute()
    }
}

pub struct cause;
