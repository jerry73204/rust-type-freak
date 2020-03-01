use crate::{
    list::{Cons, List, Nil},
    ListT,
};

// tree list

pub trait TreeList
where
    Self: List,
{
}

impl TreeList for Nil {}

impl<Child, Tail> TreeList for Cons<Child, Tail>
where
    Child: Tree,
    Tail: TreeList,
{
}

// tree

pub trait Tree
where
    Self: List,
{
}

impl Tree for Nil {}

impl<Node, Children> Tree for Cons<Node, Children> where Children: TreeList {}

// tree node

pub type TreeNode<Node, Children> = Cons<Node, Children>;

// binary tree

pub trait BinaryTree
where
    Self: Tree,
{
}

impl BinaryTree for Nil {}

impl<Node, LChild, RChild> BinaryTree for Cons<Node, ListT![LChild, RChild]>
where
    LChild: BinaryTree,
    RChild: BinaryTree,
{
}

// binary tree node

pub type BinaryTreeNode<Node, LChild, RChild> = Cons<Node, ListT![LChild, RChild]>;
