pub trait NA {
    fn na() -> Self;
}
pub struct na;

impl na {
    #[inline(always)]
    pub fn of<A: NA>() -> A {
        A::na()
    }
}