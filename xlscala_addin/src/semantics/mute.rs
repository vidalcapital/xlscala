pub trait Mute {
    fn mute() -> Self;
}
pub struct mute;

impl mute {
    #[inline(always)]
    pub fn of<A: Mute>() -> A {
        A::mute()
    }
}