use super::{LCons, LNil, TList};

// {,non-}empty list trait

/// Marks an empty [TList].
pub trait EmptyTList: TList {}

impl EmptyTList for LNil {}

/// Marks a non-empty [TList].
pub trait NonEmptyTList: TList {}

impl<Head, Tail> NonEmptyTList for LCons<Head, Tail> where Tail: TList {}
