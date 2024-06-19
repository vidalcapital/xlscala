use crate::semantics::with::With;

pub trait Successful<Arg> {
    fn successful(arg: Arg) -> Self;
}

pub struct successful;

impl <Arg> With<successful, Arg> {

    #[inline(always)]
    pub fn of<B: Successful<Arg>>(self) -> B {
        B::successful(self.with)
    }
}

impl successful {

    #[inline(always)]
    pub fn with<A>(arg: A) -> With<successful, A> {
        With::new(arg)
    }

    #[inline(always)]
    pub fn of<A: Successful<()>>() -> A {
        A::successful(())
    }

}
