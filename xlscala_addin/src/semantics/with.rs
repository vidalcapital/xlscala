use std::marker::PhantomData;

pub struct With<Tag, A> {
    pub(crate) with: A,
    phantom: PhantomData<Tag>
}

impl <Tag, A> With<Tag, A> {

    pub fn new (a: A) -> With<Tag, A> {
        With {
            with: a,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub fn with<B>(self, b: B) -> With<Tag, (A, B)> {
        With::new((self.with, b))
    }
}