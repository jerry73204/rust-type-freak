use super::{Cons, List, Nil};

// {,non-}empty list trait

/// Marks an empty [TList].
pub trait EmptyList: List {}

impl EmptyList for Nil {}

/// Marks a non-empty [TList].
pub trait NonEmptyList: List {}

impl<Head, Tail> NonEmptyList for Cons<Head, Tail> where Tail: List {}
