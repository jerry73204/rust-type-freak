//! This crate provides collections of runtime-free, type-level data structures
//! and type operators.
//!
//! The design of this crate promises these properties:
//! - **Runtime-free**: Most type operations are done in compile-time.
//! - **No concrete data**: Structs and enums contain at most
//!                         [PhantomData](std::marker::PhantomData) fields.
//! - **Less writing**: It tries to make use of type aliases to save some inks.
//!
//! # Basics
//! This crate provides type-level data structures categorized by traits, including
//! [Boolean](crate::boolean::Boolean), [TList](crate::list::TList),
//! [KVList](crate::kvlist::KVList), [Counter](crate::counter::Counter) and
//! [Maybe](crate::maybe::Maybe).
//! These types can be initialized by following:
//!
//! ```rust
//! use type_freak::{
//!     TListType,
//!     KVListType,
//!     list::{LCons, LNil},
//!     counter::{Current, Next},
//!     maybe::{UnwrapOr, Just, Nothing},
//! };
//! use typenum::{True, False, consts::*};
//!
//! type List1 = TListType![String, Option<i8>];         // impl TList trait
//! type List2 = LCons<String, LCons<Option<i8>, LNil>>;  // same as above
//! type MaybeJust = Just<f32>;                           // impl Maybe
//! type MaybeNothing = Nothing;                          // impl Maybe
//! type BoolTrue = True;                                 // impl Boolean
//! type BoolFalse = False;                               // impl Boolean
//! type Kv = KVListType![(isize, U0), (usize, U1)];     // impl KVList
//! type Cnt = Next<Next<Next<Current>>>;                 // impl Counter
//! ```
//!
//! Each data structure is manipuated either by _maps_
//! or _type operators_. The distinction is due to implementation contraints, and it
//! is encourage to use maps at first.
//! For example, you can [UnwrapOr](crate::maybe::UnwrapOr) a type that implements
//! [Maybe](crate::maybe::Maybe).
//!
//! ```rust
//! use type_freak::maybe::{UnwrapOr, Just, Nothing};
//! type Outcome1 = UnwrapOr<Just<i8>, u8>;  // Outcome1 ~= i8
//! type Outcome2 = UnwrapOr<Nothing, u8>;   // Outcome2 ~= u8
//! ```
//!
//! Actually, [UnwrapOr](crate::maybe::UnwrapOr) is an alias to apply
//! [UnwrapOrMap](crate::maybe::UnwrapOrMap) on a type. Hence, these statements
//! are equivalent but have longer syntax.
//!
//! ```rust
//! use type_freak::{
//!     maybe::{UnwrapOrMap, Just, Nothing},
//!     functional::ApplyMap,
//! };
//! type Outcome1 = ApplyMap<UnwrapOrMap<u8>, Just<i8>>;  // Outcome1 ~= i8
//! type Outcome2 = ApplyMap<UnwrapOrMap<u8>, Nothing>;   // Outcome1 ~= u8
//! ```
//!
//! # Functoinal interface
//! ## Usage
//! Both [Maybe](crate::maybe::Maybe) and [TList](crate::list::TList) allows you to
//! pass a map for data manipulation. In this example, we pass
//! [AddOneMap](crate::numeric::AddOneMap) that increases input typed integer
//! by one.
//!
//! ```rust
//! use type_freak::{
//!     TListType,
//!     list::LMap,
//!     maybe::{Just, Nothing},
//!     numeric::AddOneMap,
//! };
//! use typenum::consts::*;
//!
//! type Out1 = LMap<TListType![U3, U2, U5], AddOneMap>;
//! // Out1 ~= TListType![U4. U3. U5]
//!
//! type Out2 = LMap<Just<U7>, AddOneMap>;
//! // Out2 ~= Just<U8>
//!
//! type Out3 = LMap<Nothing, AddOneMap>;
//! // Out3 ~= Nothing
//! ```
//!
//! [TList](crate::list::TList) also provides [LFilter](crate::list::LFilter) and
//! [LScan](crate::list::LScan) that acts like [Iterator](std::iter::Iterator)'s
//! [filter](std::iter::Iterator::filter) and [scan](std::iter::Iterator::scan).
//! Their detailed usage is out of scope here.
//!
//! ## Roll your own [Map](crate::functional::Map)
//! You can define your own map by implementing [Map](crate::functional::Map)
//! trait on your type. Here we make a map that boxes the input type.
//!
//! ```rust
//! use type_freak::functional::Map;
//! struct BoxMap;
//!
//! impl<Input> Map<Input> for BoxMap {
//!     type Output = Box<Input>;
//! }
//!
//! ```
//!
//! Use [ApplyMap](crate::functional::ApplyMap) to apply your map on a type.
//!
//! ```ignore
//! type Out = ApplyMap<BoxMap, String>;  // Out ~= Box<String>
//! ```
//!
//! To make sure it works as expected, the crate has a special operator
//! [IfSameOutput](crate::control::IfSameOutput) to let us write static asserttion.
//! If it does its job, the following code should compile without errors.
//!
//! ```rust
//! use type_freak::{
//!     functional::{Map, ApplyMap},
//!     control::IfSameOutput
//! };
//!
//! struct BoxMap;
//!
//! impl<Input> Map<Input> for BoxMap {
//!     type Output = Box<Input>;
//! }
//!
//! type Out = ApplyMap<BoxMap, String>;  // Out ~= Box<String>
//! type Assert = IfSameOutput<(), Out, Box<String>>;
//!
//! fn assert() {
//!     let _: Assert = ();
//! }
//!
//! ```
//!
//! # Maps vs type operators
//! Most operations in this crate are in by [Map](crate::functional::Map)s,
//! while a handful of them are _type operators_. To be exact,
//!
//! - Maps are structs that implements [Map](crate::functional::Map) trait.
//! - Type operators are traits that has `Output` associated type.
//!
//! They serves for the same purpose. Since recursive associated types only works on
//! trait type operators, some of the operators have both a trait and a map.
//!
//! For example,
//! [LInsertAtOp](crate::list::LInsertAtOp) is a trait that inserts a new type to
//! [TList](crate::list::TList). [LInsertAtOpOutput](crate::list::LInsertAtOpOutput)
//! is type alias to its `Output` associated type.
//! [LInsertAtMap](crate::list::LInsertAtMap) is the map wrapping around
//! above trait, and [LInsertAt](crate::list::LInsertAt) is an alias to apply the map.
//!
//! # Naming conventions
//! Each traits, structs and type aliases serves their own purpose. The naming of
//! these primitives tell you the usage and how the outcome could be.
//!
//! ## Traits as _type operators_
//! The name has pattern `*Op`, such as [FirstOfOp](crate::list::LLengthOp).
//! Most _type operators_ have a `Output` associated type to represent the outcome
//! of type transformation. You can obtain the outcome by trait casting. For example,
//!
//! ```ignore
//! type Outcome = <TListType![i8, i16] as LLengthOp<>>::Output;  // Outcome ~= U2
//! ```
//!
//! Due to the cumbersome syntax, most type operators have a corresponding type
//! alias to capture the output. For example,
//!
//! ```ignore
//! type Outcome = LLengthOpOutput<TListType![i8, i16]>;  // Output ~= U2
//! ```
//!
//! ## Traits as _markers_
//! Marker traits are usually placed under `marker` namespace. For example,
//! [type_freak::list::marker::EmptyTList](crate::list::marker::EmptyTList).
//! These traits are mostly shown in trait bounds.
//!
//! ## Structs as _maps_
//! A map is a special struct that implements [Map](crate::functional::Map).
//! The name ends in `*Map`, such as [LLengthMap](crate::list::LLengthMap).
//! It works like _type operators_, and it can be applied to [Maybe](crate::maybe::Maybe)
//! and [TList](crate::list::TList). Most type operations have a map interface,
//! either by wrapping around a _type operator_ trait or by standalone definition.
//!
//! The crate provides [ApplyMap\<Func, Input\>](crate::functional::ApplyMap)
//! type alias to apply a map on a type. For example,
//!
//! ```ignore
//! type Outcome = ApplyMap<LLengthMap, TListType![u8, u16]>;  // Outcome ~= U2
//! // Same as `LLengthMap as Map<TListType![u8, u16]>`
//! ```
//!
//! Also, type aliases are available to save the pen and ink.
//!
//! ```ignore
//! type Outcome = LLength<TListType![u8, u16]>;  // Outcome ~= U2
//! ```
//!
//! ## Type aliases as _type operator aliases_
//!
//! Most alias names ends in `*OpOutput`. For example, the name
//! [LLengthOpOutput\<List\>](crate::list::LLengthOpOutput) is the alias of
//! `<List as LLengthOp>::Output`.
//!
//! ## Type aliases as _map aliases_
//! It is a short hand syntax to apply a map on a type. The name has no
//! special suffix. For example, [LLength\<List\>](crate::list::LLength)
//! is alias of `ApplyMap<LLengthMap, List>`.
//!
//!
//!

#![feature(vec_remove_item)]

// pub mod boolean;
pub mod control;
pub mod counter;
pub mod functional;
pub mod kvlist;
pub mod list;
pub mod tree;
// pub mod maybe;
pub mod numeric;
// pub mod tuple;
pub mod fraction;
