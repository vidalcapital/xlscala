
pub trait Empty {
    fn empty() -> Self;
}

pub struct empty;

impl empty {
    #[inline(always)]
    pub fn of<A: Empty>() -> A {
        A::empty()
    }
}