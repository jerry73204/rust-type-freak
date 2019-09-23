use std::marker::PhantomData;

// maybe def

/// A trait analogous to [Option](std::option::Option).
pub trait Maybe {}

// just def

pub struct Just<T> {
    _phantom: PhantomData<T>,
}

impl<T> Maybe for Just<T> {}

// nothing def

pub struct Nothing;

impl Maybe for Nothing {}

// unwrap op

/// A type operator that unwraps [Just<T>](Just).
pub trait Unwrap
where
    Self: Maybe,
{
    type Output;
}

impl<T> Unwrap for Just<T> {
    type Output = T;
}

pub type UnwrapOutput<T> = <T as Unwrap>::Output;
