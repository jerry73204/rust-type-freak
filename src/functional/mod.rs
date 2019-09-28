//! Functional primitives like [Functor](crate::functional::Functor) and
//! [Compose](crate::functional::Compose).

mod applicative;
mod fmap;
mod functor;

pub use applicative::*;
pub use fmap::*;
pub use functor::*;
