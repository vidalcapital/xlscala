use crate::semantics::with::With;

pub trait Cons<Arg> {
    fn construct(arg: Arg) -> Self;
}

pub struct cons;

impl <Arg> With<cons, Arg> {

    #[inline(always)]
    pub fn of<B: Cons<Arg>>(self) -> B {
        B::construct(self.with)
    }
}

impl cons {
    #[inline(always)]
    pub fn with<A>(arg: A) -> With<cons, A> {
        With::new(arg)
    }

}
