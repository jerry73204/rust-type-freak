// list

/// Represents a typed list constructed by [LCons] and [LNil].
pub trait List {}

// intermediate node

/// Represents an intermediate node.
pub struct Cons<Head, Tail>(Head, Tail)
where
    Tail: List;

impl<Head, Tail> List for Cons<Head, Tail> where Tail: List {}

// end of list

/// Represents the end of list.
pub struct Nil;

impl List for Nil {}
