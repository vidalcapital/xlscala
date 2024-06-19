
pub trait Builder {
    type Builder;
    fn builder() -> Self::Builder;
}

pub struct builder;

impl builder {

    #[inline(always)]
    pub fn of<A: Builder>() -> A::Builder {
        A::builder()
    }
}