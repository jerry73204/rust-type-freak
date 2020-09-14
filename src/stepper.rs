use crate::{
    common::*,
    list::base::{Cons, List, Nil},
};

mod base {
    use super::*;

    pub trait Stepper
    where
        Self: List,
    {
    }

    pub type Next<Tail> = Cons<(), Tail>;
    pub type Curr = Nil;

    impl<Tail> Stepper for Next<Tail> where Tail: Stepper {}
    impl Stepper for Nil {}
}

pub use base::*;

mod macros {
    // TODO
    // #[macro_export]
    // macro_rules! Stepper {
    //     ($size:literal) => { $crate::counter::op_aliases::ToStepper<$size> };
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::op_aliases::*;

    // type Assert1 = AssertSame<Stepper![U0], Nil, ()>;
    // type Assert2 = AssertSame<Stepper![U1], Step<Nil>, ()>;
    // type Assert3 = AssertSame<Stepper![U2], Step<Step<Nil>>, ()>;
    // type Assert4 = AssertSame<Stepper![U3], Step<Step<Step<Nil>>>, ()>;

    #[test]
    fn stepper_test() {
        // let _: Assert1 = ();
        // let _: Assert2 = ();
        // let _: Assert3 = ();
        // let _: Assert4 = ();
    }
}
