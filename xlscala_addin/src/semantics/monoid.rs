use std::marker::PhantomData;
use std::ops::Deref;
use crate::semantics::like::Like;

pub trait Monoid<Op> {
    fn mute() -> Self;
    fn is_mute(&self) -> bool;
    unsafe fn unsafe_op(left: Self, right: Self) -> Self;

    #[inline(always)]
    fn op(left: Self, right: Self) -> Self
    where Self: Clone {
        if left.is_mute() {
            right
         } else {
            if right.is_mute() {
                left
            } else {
                unsafe {
                    Self::unsafe_op(left, right)
                }
            }
        }
    }
}

pub struct MonoidBox<Op, A: Monoid<Op>>{
    pub value: A,
    marker: PhantomData<Op>
}

impl <Op, A: Monoid<Op>> MonoidBox<Op, A> {

    pub fn new(value: A) -> MonoidBox<Op, A> {
        MonoidBox {
            value: value,
            marker: PhantomData
        }
    }

    #[inline(always)]
    pub fn unwrap(self) -> A {
        self.value
    }

    #[inline(always)]
    pub fn with<B: Like<Op, A>>(&mut self, b: &B) -> &Self {
        self.value = A::op(*self.value, b.like());
        self
    }
 }

impl <Op, A: Monoid<Op>> Deref for MonoidBox<Op, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/*pub trait MonoidBoxOps {
    pub fn
}

impl <A> MonoidBoxOps for A {

}*/