// pub trait Monoid {}

// pub trait Identity: Monoid {}

// pub trait ComposeKind {}

// pub struct LeftIdentity;

// impl ComposeKind for LeftIdentity {}

// pub struct RightIdentity;

// impl ComposeKind for RightIdentity {}

// pub struct BothIdentity;

// impl ComposeKind for BothIdentity {}

// pub trait Compose<Rhs, Kind>
// where
//     Rhs: Monoid,
//     Kind: ComposeKind,
//     Self: Monoid,
//     Self::Out: Monoid,
// {
//     type Out;
// }

// pub type ComposeOut<Lhs, Rhs, Kind> = <Lhs as Compose<Rhs, Kind>>::Out;

// impl<Lhs, Rhs> Compose<Rhs, LeftIdentity> for Lhs
// where
//     Lhs: Identity,
//     Rhs: Monoid,
// {
//     type Out = Rhs;
// }

// impl<Lhs, Rhs> Compose<Rhs, RightIdentity> for Lhs
// where
//     Lhs: Monoid,
//     Rhs: Identity,
// {
//     type Out = Lhs;
// }

// impl<Lhs, Rhs> Compose<Rhs, BothIdentity> for Lhs
// where
//     Lhs: Identity,
//     Rhs: Identity,
// {
//     type Out = Lhs;
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     pub struct A;
//     pub struct B;
//     pub struct E;

//     impl A {
//         pub fn new() -> Self {
//             Self
//         }
//     }

//     impl B {
//         pub fn new() -> Self {
//             Self
//         }
//     }

//     impl E {
//         pub fn new() -> Self {
//             Self
//         }
//     }

//     impl Monoid for A {}
//     impl Monoid for B {}
//     impl Monoid for E {}
//     impl Identity for E {}

//     type X<Kind> = ComposeOut<A, E, Kind>;
//     type Y<Kind> = ComposeOut<E, B, Kind>;
//     type Z<Kind> = ComposeOut<E, E, Kind>;
//     type W<Kind> = ComposeOut<A, B, Kind>;

//     #[test]
//     fn monoid_test() {
//         let _: X<_> = X::new();
//         let _: Y<_> = Y::new();
//         let _: Z<_> = Z::<_>::new();
//         // let _: W<_> = W::new();
//     }
// }
