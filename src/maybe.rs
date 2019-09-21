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

/// A trait operator that unwraps [Just<T>](Just).
pub trait Unwrap
where
    Self: Maybe,
{
    type Out;
}

impl<T> Unwrap for Just<T> {
    type Out = T;
}

pub type UnwrapOut<T> = <T as Unwrap>::Out;
