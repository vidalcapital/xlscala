use std::marker::PhantomData;

pub mod compiletime {
    use std::marker::PhantomData;

    pub trait Converter<S> {
        #[inline(always)]
        fn convert(s: S) -> Option<S>;
    }

    pub struct With<S, A: Converter<S>> {
        pub(crate) phantom_s: PhantomData<S>,
        pub(crate) phantom_a: PhantomData<A>
    }

    impl<S, A: Converter<S>> With<S, A> {
        #[inline(always)]
        pub fn with<B: Converter<S>>(self) -> With<S, Cons<S, A, B>> {
            With {
                phantom_s: PhantomData,
                phantom_a: PhantomData
            }
        }

        pub fn convert(self, s: S) -> Option<S> {
            A::convert(s)
        }
    }

    pub struct Mute<S> {
        phantom_s: PhantomData<S>
    }

    impl <S> Converter<S> for Mute<S> {
        fn convert(a: S) -> Option<S> {
            Some(a)
        }
    }

    pub struct Cons<S, A: Converter<S>, B: Converter<S>> {
        phantom_s: PhantomData<S>,
        phantom_a: PhantomData<A>,
        phantom_b: PhantomData<B>
    }

    impl <S, A: Converter<S>, B: Converter<S>> Converter<S> for Cons<S, A, B> {
        fn convert(s: S) -> Option<S> {
            match B::convert(s) {
                Some(value) => A::convert(value),
                None => None
            }
        }
    }


}
struct converter;

impl converter {

    #[inline(always)]
    pub fn of<S>() -> compiletime::With<S, compiletime::Mute<S>> {
        compiletime::With {
            phantom_s: PhantomData,
            phantom_a: PhantomData
        }
    }

}