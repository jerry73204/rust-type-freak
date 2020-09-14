pub trait Same<Lhs, Rhs, Out = ()> {
    type Output;
}

impl<T, Out> Same<T, T, Out> for () {
    type Output = Out;
}

pub type SameOp<Lhs, Rhs> = <() as Same<Lhs, Rhs>>::Output;
pub type SameExOp<Lhs, Rhs, Out> = <() as Same<Lhs, Rhs, Out>>::Output;
