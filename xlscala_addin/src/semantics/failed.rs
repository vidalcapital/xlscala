use crate::semantics::successful::Successful;
use crate::semantics::with::With;

pub trait Failed<Arg> {
    fn failed(a: Arg) -> Self;
}

pub struct failed;


impl <Arg> With<failed, Arg> {

    #[inline(always)]
    pub fn of<B: Failed<Arg>>(self) -> B {
        B::failed(self.with)
    }
}

impl failed {

    #[inline(always)]
    pub fn with<A>(arg: A) -> With<failed, A> {
        With::new(arg)
    }

    #[inline(always)]
    pub fn of<A: Failed<()>>() -> A {
        A::failed(())
    }

}