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
//! TListType! {u8, u16, u32}
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
//! type List1 = TListType! {u8, u16, u32};
//!
//! type List2 = LPrependOutput<List1, u64>;
//! // List2 ~= TListType! {u64, u8, u16, u32}
//! // is alias of <List1 as LPrepend<List1, u64>>::Output
//!
//! type List3<Index1> = LRemoveAtOutput<List2, u16, Index1>;
//! // List3<_> ~= TListType! {u64, u8, u32}
//!
//! type List4<Index1> = LAppendOutput<List3<Index1>, f32>;
//! // List4 ~= TListType! {u64, u8, u32, f32}
//!
//! type List5<Index1, Index2> = LInsertAtOutput<List4<Index1>, u8, f64, Index2>;
//! // List5 ~= TListType! {u64, u8, f64, u32, f32}
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
//! The [EmptyTList](crate::list::EmptyTList) and [NonEmptyTList](crate::list::NonEmptyTList)
//! traits can be used in trait bounds. Suppose you wish to accept a non-empty
//! [TList](crate::list::TList) type:
//!
//! ```ignore
//! trait ExampleTrait<List: NonEmptytList> { /* ... */ }
//! ```
//! ## Numeric type operators
//! [LReduceMax](crate::list::LReduceMax), [LReduceMin](crate::list::LReduceMin),
//! [LReduceSum](crate::list::LReduceSum) and [LReduceProduct](crate::list::LReduceProduct)
//! assume all contained types are [typenum] typed numbers. You may `use typenum::consts::*`
//! to work with [typenum] constants.
//!
//! ```rust
//! use type_freak::{TListType, list::LReduceSumOutput};
//! use typenum::consts::*;
//!
//! type Value = LReduceSumOutput<TListType! {P3, N5, Z0}>;  // Value ~= P2
//! ```
//!
//! The [LToUsizeVec](crate::list::LToUsizeVec) provides a
//! [to_usize_vec](crate::list::LToUsizeVec::to_usize_vec) to build concrete
//! `Vec<usize>` type.
//!
//! ```ignore
//! // Gets vec![3, -5, 0]
//! let values = <TListType! {P3, N5, Z0} as LToUsizeVec>::to_usize_vec();
//! ```

mod functional;
mod indexing;
mod insert;
mod macros;
mod marker;
mod misc;
mod reduction;
mod remove;
mod reverse;

pub use functional::*;
pub use indexing::*;
pub use insert::*;
pub use macros::*;
pub use marker::*;
pub use misc::*;
pub use reduction::*;
pub use remove::*;
pub use reverse::*;

use crate::TListType;
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

// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{control::IfSameOutput, TListType};
    use typenum::consts::*;

    type AssertSame<Lhs, Rhs> = IfSameOutput<(), Lhs, Rhs>;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;

    type EmptyList = TListType! {};
    type SomeList = TListType! {A, B, C};
    type AnotherList = TListType! {D, E};

    // prepend empty list
    type Assert1 = AssertSame<LPrependOutput<EmptyList, A>, TListType! {A}>;

    // append empty list
    type Assert2 = AssertSame<LAppendOutput<EmptyList, D>, TListType! {D}>;

    // prepend non-empty list
    type Assert3 = AssertSame<LPrependOutput<SomeList, D>, TListType! {D, A, B, C}>;

    // append non-empty list
    type Assert4 = AssertSame<LAppendOutput<SomeList, D>, TListType! {A, B, C, D}>;

    // insert in middle
    type Assert5<Idx> = AssertSame<LInsertAtOutput<SomeList, D, B, Idx>, TListType! {A, B, D, C}>;

    // insert at end
    type Assert6<Idx> = AssertSame<LInsertAtOutput<SomeList, D, C, Idx>, TListType! {A, B, C, D}>;

    // remove
    type Assert7<Idx> = AssertSame<LRemoveAtOutput<SomeList, B, Idx>, TListType! {A, C}>;

    // remove multiple items
    type Assert8<Idx> =
        AssertSame<LRemoveManyOutput<SomeList, TListType! {A, C}, Idx>, TListType! {B}>;

    // remove until empty
    type Assert9<Idx> =
        AssertSame<LRemoveManyOutput<SomeList, TListType! {B, A, C}, Idx>, TListType! {}>;

    // reverse list
    type Assert10 = AssertSame<LReverseOutput<SomeList>, TListType! {C, B, A}>;

    // assert identical set of items
    type Assert11<Idx> = LSetEqualOutput<SomeList, TListType! {C, A, B}, Idx>;

    // concat
    type Assert12 = AssertSame<LConcatOutput<SomeList, AnotherList>, TListType! {A, B, C, D, E}>;

    // index of tiem
    type Assert13<Idx> = AssertSame<LIndexOfIndex<SomeList, A, Idx>, U0>;
    type Assert14<Idx> = AssertSame<LIndexOfIndex<SomeList, B, Idx>, U1>;
    type Assert15<Idx> = AssertSame<LIndexOfIndex<SomeList, C, Idx>, U2>;

    // index of multiple items
    type Indexes<Idx> = LIndexOfManyIndexes<SomeList, TListType! {C, A, B}, Idx>;
    type Assert16<Idx> = AssertSame<Indexes<Idx>, TListType! {U2, U0, U1}>;

    #[test]
    fn tlist_test() {
        let _: Assert1 = ();
        let _: Assert2 = ();
        let _: Assert3 = ();
        let _: Assert4 = ();
        let _: Assert5<_> = ();
        let _: Assert6<_> = ();
        let _: Assert7<_> = ();
        let _: Assert8<_> = ();
        let _: Assert9<_> = ();
        let _: Assert10 = ();
        let _: Assert11<_> = ();
        let _: Assert12 = ();
        let _: Assert13<_> = ();
        let _: Assert14<_> = ();
        let _: Assert15<_> = ();
        let _: Assert16<_> = ();

        assert_eq!(<Indexes<_> as LToUsizeVec>::to_usize_vec(), &[2, 0, 1]);
    }
}
