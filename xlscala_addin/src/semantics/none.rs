pub trait None {
    fn none() -> Self;
}
pub struct none;

impl none {
    #[inline(always)]
    pub fn of<A: None>() -> A {
        A::none()
    }
}