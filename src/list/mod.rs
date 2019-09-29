//! Typed list that supports insertion, removal and look-up.
//!
//! ## Type construction
//! The [TList](crate::list::TList) trait represents a typed list of arbitrary types.
//! The type [LCons](crate::list::LCons) forms intermediate nodes, while
//! [LNil](crate::list::LNil) type marks the end of list. For a list of `u8`, `u16`
//! and `u32` types:
//!
//! ```ignore
//! LCons<u8, LCons<u16, LCons<u32, LNil>>>
//! ```
//!
//! Most of the time you don't need to write in this cumbersome way.
//! The [TListType] macro let you write in more compact syntax. For example,
//!
//! ```ignore
//! TListType![u8, u16, u32]
//! ```
//!
//! ## List manipuation
//! The module ships a collection of _type operators_ to manipulate lists,
//! including [LPrepend](crate::list::LPrepend), [LAppend](crate::list::LAppend),
//! [LInsertAt](crate::list::LInsertAt), [LRemoveAt](crate::list::LRemoveAt).
//! As the name explains itself, you can append or prepend a type to this list,
//! insert a new type after a some type, or remove a specific type. We can work
//! it out by their type aliases for convenience.
//!
//! ```rust
//! use type_freak::{TListType, list::*};
//!
//! type List1 = TListType![u8, u16, u32];
//!
//! type List2 = LPrepend<List1, u64>;
//! // List2 ~= TListType![u64, u8, u16, u32]
//! // is alias of <List1 as LPrepend<List1, u64>>::Output
//!
//! type List3<Index1> = LRemoveAt<List2, u16, Index1>;
//! // List3<_> ~= TListType![u64, u8, u32]
//!
//! type List4<Index1> = LAppend<List3<Index1>, f32>;
//! // List4 ~= TListType![u64, u8, u32, f32]
//!
//! type List5<Index1, Index2> = LInsertAt<List4<Index1>, u8, f64, Index2>;
//! // List5 ~= TListType![u64, u8, f64, u32, f32]
//! ```
//!
//! As shown in the example, [LInsertAt](crate::list::LInsertAt),
//! [LRemoveAt](crate::list::LRemoveAt) along with other type operators
//! have a special `Index` generic type argument. It is necessary for
//! list traversal. Most of the time we can leave it undetermined.
//! It can be inferred by compiler when constructing concrete type.
//!
//! ```ignore
//! let _ = List5::<_, _>::new();
//! ```
//!
//! ## Marker traits
//! The [EmptyTList](crate::list::marker::EmptyTList) and [NonEmptyTList](crate::list::marker::NonEmptyTList)
//! traits can be used in trait bounds. Suppose you wish to accept a non-empty
//! [TList](crate::list::TList) type:
//!
//! ```ignore
//! trait ExampleTrait<List: NonEmptytList> { /* ... */ }
//! ```
//! ## Numeric type operators
//! [LReduceMax](crate::list::LReduceMax), [LReduceMin](crate::list::LReduceMin),
//! [LReduceSum](crate::list::LReduceSum) and [LReduceProd](crate::list::LReduceProd)
//! assume all contained types are [typenum] typed numbers. You may `use typenum::consts::*`
//! to work with [typenum] constants.
//!
//! ```rust
//! use type_freak::{TListType, list::LReduceSum};
//! use typenum::consts::*;
//!
//! type Value = LReduceSum<TListType![P3, N5, Z0]>;  // Value ~= P2
//! ```
//!
//! The [LToUsizeVec](crate::list::LToUsizeVec) provides a
//! [to_usize_vec](crate::list::LToUsizeVec::to_usize_vec) to build concrete
//! `Vec<usize>` type.
//!
//! ```ignore
//! // Gets vec![3, -5, 0]
//! let values = <TListType![P3, N5, Z0] as LToUsizeVec>::to_usize_vec();
//! ```

mod functional;
mod indexing;
mod insert;
mod macros;
pub mod marker;
mod misc;
mod reduction;
mod remove;
mod zipping;

pub use functional::*;
pub use indexing::*;
pub use insert::*;
pub use macros::*;
pub use misc::*;
pub use reduction::*;
pub use remove::*;
pub use zipping::*;

use std::marker::PhantomData;

// list

/// Represents a typed list constructed by [LCons] and [LNil].
pub trait TList {}

// intermediate node

/// Represents an intermediate node.
pub struct LCons<Head, Tail>
where
    Tail: TList,
{
    _phantom: PhantomData<(Head, Tail)>,
}

impl<Head, Tail> LCons<Head, Tail>
where
    Tail: TList,
{
    pub fn new() -> LCons<Head, Tail> {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Head, Tail> TList for LCons<Head, Tail> where Tail: TList {}

// end of list

/// Represents the end of list.
pub struct LNil;

impl TList for LNil {}
