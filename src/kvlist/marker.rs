use super::{KVCons, KVList, KVNil};

/// A marker trait that marks the empty [KVList].
pub trait EmptyKVList: KVList {}

impl EmptyKVList for KVNil {}

/// A marker trait that marks non-empty [KVList].
pub trait NonEmptyKVList: KVList {}

impl<Key, Value, Tail> NonEmptyKVList for KVCons<Key, Value, Tail> where Tail: KVList {}
