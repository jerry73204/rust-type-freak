//! Functional primitives like [Map](crate::functional::Map) and
//! [Compose](crate::functional::Compose).

mod applicative;
mod map;

pub use applicative::*;
pub use map::*;
