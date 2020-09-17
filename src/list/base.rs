// list

/// Represents a typed list constructed by [LCons] and [LNil].
pub trait List {}

// intermediate node

/// Represents an intermediate node.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cons<Head, Tail>
where
    Tail: List,
{
    pub head: Head,
    pub tail: Tail,
}

impl<Head, Tail> List for Cons<Head, Tail> where Tail: List {}

// end of list

/// Represents the end of list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nil;

impl List for Nil {}

/// Marks an empty [TList].
pub trait EmptyList: List {}

impl EmptyList for Nil {}

/// Marks a non-empty [TList].
pub trait NonEmptyList: List {}

impl<Head, Tail> NonEmptyList for Cons<Head, Tail> where Tail: List {}
