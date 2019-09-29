//! A typed list of key-value pairs.

mod access;
mod insert;
mod macros;
pub mod marker;
mod misc;
mod remove;

pub use access::*;
pub use insert::*;
pub use misc::*;
pub use remove::*;

use crate::list::{LCons, LNil, TList};

// list

/// The trait represents a list of key-value pairs.
pub trait KVList
where
    Self: TList,
{
}

/// A node of [KVList].
pub type KVCons<Key, Value, Tail> = LCons<(Key, Value), Tail>;

impl<Key, Value, Tail> KVList for KVCons<Key, Value, Tail> where Tail: KVList {}

/// The ending node of [KVList].
pub type KVNil = LNil;

impl KVList for LNil {}
