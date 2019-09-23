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
//     Self::Output: <Monoid,
// {
//     type Output;
// }

// pub type ComposeOutput<Lhs, Rhs, Kind> = <Lhs as Compose<Rhs, Kind>>::Output;

// impl<Lhs, Rhs> Compose<Rhs, LeftIdentity> for Lhs
// where
//     Lhs: Identity,
//     Rhs: Monoid,
// {
//     type Output = Rhs;
// }

// impl<Lhs, Rhs> Compose<Rhs, RightIdentity> for Lhs
// where
//     Lhs: Monoid,
//     Rhs: Identity,
// {
//     type Output = Lhs;
// }

// impl<Lhs, Rhs> Compose<Rhs, BothIdentity> for Lhs
// where
//     Lhs: Identity,
//     Rhs: Identity,
// {
//     type Output = Lhs;
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

//     type X<Kind> = ComposeOutput<A, E, Kind>;
//     type Y<Kind> = ComposeOutput<E, B, Kind>;
//     type Z<Kind> = ComposeOutput<E, E, Kind>;
//     type W<Kind> = ComposeOutput<A, B, Kind>;

//     #[test]
//     fn monoid_test() {
//         let _: X<_> = X::new();
//         let _: Y<_> = Y::new();
//         let _: Z<_> = Z::<_>::new();
//         // let _: W<_> = W::new();
//     }
// }
