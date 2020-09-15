pub use markers::*;
pub use ops::*;

mod markers {
    pub trait Tuple0 {}
    impl Tuple0 for () {}

    pub trait Tuple1 {}
    impl<E0> Tuple1 for (E0,) {}

    pub trait Tuple2 {}
    impl<E0, E1> Tuple2 for (E0, E1) {}

    pub trait Tuple3 {}
    impl<E0, E1, E2> Tuple3 for (E0, E1, E2) {}

    pub trait Tuple4 {}
    impl<E0, E1, E2, E3> Tuple4 for (E0, E1, E2, E3) {}

}

mod ops {
    pub trait Get0 {
        type Output;
    }

    pub trait Get1 {
        type Output;
    }

    pub trait Get2 {
        type Output;
    }

    pub trait Get3 {
        type Output;
    }

    // 1-tuple

    impl<E0> Get0 for (E0,) {
        type Output = E0;
    }

    // 2-tuple

    impl<E0, E1> Get0 for (E0, E1) {
        type Output = E0;
    }

    impl<E0, E1> Get1 for (E0, E1) {
        type Output = E1;
    }

    // 3-tuple

    impl<E0, E1, E2> Get0 for (E0, E1, E2) {
        type Output = E0;
    }

    impl<E0, E1, E2> Get1 for (E0, E1, E2) {
        type Output = E1;
    }

    impl<E0, E1, E2> Get2 for (E0, E1, E2) {
        type Output = E2;
    }

    // 4-tuple

    impl<E0, E1, E2, E3> Get0 for (E0, E1, E2, E3) {
        type Output = E0;
    }

    impl<E0, E1, E2, E3> Get1 for (E0, E1, E2, E3) {
        type Output = E1;
    }

    impl<E0, E1, E2, E3> Get2 for (E0, E1, E2, E3) {
        type Output = E2;
    }

    impl<E0, E1, E2, E3> Get3 for (E0, E1, E2, E3) {
        type Output = E3;
    }
}
