use crate::common::*;

// macros

macro_rules! typenum_unsigned_impl {
    ($ty:ty, $output:ident) => {
        impl Add<UTerm> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: UTerm) -> Self::Output {
                Dyn(self.0.add(UTerm::$output))
            }
        }

        impl Add<Dyn<$ty>> for UTerm {
            type Output = Dyn<$ty>;

            fn add(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.add(rhs.0))
            }
        }

        impl<U, B> Add<UInt<U, B>> for Dyn<$ty>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: UInt<U, B>) -> Self::Output {
                Dyn(self.0.add(UInt::<U, B>::$output))
            }
        }

        impl<U, B> Add<Dyn<$ty>> for UInt<U, B>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn add(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.add(rhs.0))
            }
        }

        impl Sub<UTerm> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: UTerm) -> Self::Output {
                Dyn(self.0.sub(UTerm::$output))
            }
        }

        impl Sub<Dyn<$ty>> for UTerm {
            type Output = Dyn<$ty>;

            fn sub(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.sub(rhs.0))
            }
        }

        impl<U, B> Sub<UInt<U, B>> for Dyn<$ty>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: UInt<U, B>) -> Self::Output {
                Dyn(self.0.sub(UInt::<U, B>::$output))
            }
        }

        impl<U, B> Sub<Dyn<$ty>> for UInt<U, B>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn sub(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.sub(rhs.0))
            }
        }

        impl Mul<UTerm> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: UTerm) -> Self::Output {
                Dyn(self.0.mul(UTerm::$output))
            }
        }

        impl Mul<Dyn<$ty>> for UTerm {
            type Output = Dyn<$ty>;

            fn mul(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.mul(rhs.0))
            }
        }

        impl<U, B> Mul<UInt<U, B>> for Dyn<$ty>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: UInt<U, B>) -> Self::Output {
                Dyn(self.0.mul(UInt::<U, B>::$output))
            }
        }

        impl<U, B> Mul<Dyn<$ty>> for UInt<U, B>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn mul(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.mul(rhs.0))
            }
        }

        impl Div<UTerm> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn div(self, _rhs: UTerm) -> Self::Output {
                Dyn(self.0.div(UTerm::$output))
            }
        }

        impl Div<Dyn<$ty>> for UTerm {
            type Output = Dyn<$ty>;

            fn div(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.div(rhs.0))
            }
        }

        impl<U, B> Div<UInt<U, B>> for Dyn<$ty>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn div(self, _rhs: UInt<U, B>) -> Self::Output {
                Dyn(self.0.div(UInt::<U, B>::$output))
            }
        }

        impl<U, B> Div<Dyn<$ty>> for UInt<U, B>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn div(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.div(rhs.0))
            }
        }
    };
}

macro_rules! typenum_signed_impl {
    ($ty:ty, $output:ident) => {
        impl<U> Add<PInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: PInt<U>) -> Self::Output {
                Dyn(self.0.add(PInt::<U>::$output))
            }
        }

        impl<U> Add<Dyn<$ty>> for PInt<U>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn add(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.add(rhs.0))
            }
        }

        impl<U> Add<NInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: NInt<U>) -> Self::Output {
                Dyn(self.0.add(NInt::<U>::$output))
            }
        }

        impl<U> Add<Dyn<$ty>> for NInt<U>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn add(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.add(rhs.0))
            }
        }

        impl Add<Z0> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: Z0) -> Self::Output {
                Dyn(self.0.add(Z0::$output))
            }
        }

        impl Add<Dyn<$ty>> for Z0 {
            type Output = Dyn<$ty>;

            fn add(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.add(rhs.0))
            }
        }

        impl<U> Sub<PInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: PInt<U>) -> Self::Output {
                Dyn(self.0.sub(PInt::<U>::$output))
            }
        }

        impl<U> Sub<Dyn<$ty>> for PInt<U>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn sub(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.sub(rhs.0))
            }
        }

        impl<U> Sub<NInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: NInt<U>) -> Self::Output {
                Dyn(self.0.sub(NInt::<U>::$output))
            }
        }

        impl<U> Sub<Dyn<$ty>> for NInt<U>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn sub(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.sub(rhs.0))
            }
        }

        impl Sub<Z0> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: Z0) -> Self::Output {
                Dyn(self.0.sub(Z0::$output))
            }
        }

        impl Sub<Dyn<$ty>> for Z0 {
            type Output = Dyn<$ty>;

            fn sub(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.sub(rhs.0))
            }
        }

        impl<U> Mul<PInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: PInt<U>) -> Self::Output {
                Dyn(self.0.mul(PInt::<U>::$output))
            }
        }

        impl<U> Mul<Dyn<$ty>> for PInt<U>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn mul(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.mul(rhs.0))
            }
        }

        impl<U> Mul<NInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: NInt<U>) -> Self::Output {
                Dyn(self.0.mul(NInt::<U>::$output))
            }
        }

        impl<U> Mul<Dyn<$ty>> for NInt<U>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn mul(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.mul(rhs.0))
            }
        }

        impl Mul<Z0> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: Z0) -> Self::Output {
                Dyn(self.0.mul(Z0::$output))
            }
        }

        impl Mul<Dyn<$ty>> for Z0 {
            type Output = Dyn<$ty>;

            fn mul(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.mul(rhs.0))
            }
        }

        impl<U> Div<PInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn div(self, _rhs: PInt<U>) -> Self::Output {
                Dyn(self.0.div(PInt::<U>::$output))
            }
        }

        impl<U> Div<Dyn<$ty>> for PInt<U>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn div(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.div(rhs.0))
            }
        }

        impl<U> Div<NInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn div(self, _rhs: NInt<U>) -> Self::Output {
                Dyn(self.0.div(NInt::<U>::$output))
            }
        }

        impl<U> Div<Dyn<$ty>> for NInt<U>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn div(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.div(rhs.0))
            }
        }

        impl Div<Z0> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn div(self, _rhs: Z0) -> Self::Output {
                Dyn(self.0.div(Z0::$output))
            }
        }

        impl Div<Dyn<$ty>> for Z0 {
            type Output = Dyn<$ty>;

            fn div(self, rhs: Dyn<$ty>) -> Self::Output {
                Dyn(Self::$output.div(rhs.0))
            }
        }
    };
}

// type

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dyn<T>(pub T);

// general ipls

impl<T> Add<Dyn<T>> for Dyn<T>
where
    T: Add<T>,
{
    type Output = Dyn<<T as Add<T>>::Output>;

    fn add(self, rhs: Dyn<T>) -> Self::Output {
        Dyn(self.0.add(rhs.0))
    }
}

impl<T> Sub<Dyn<T>> for Dyn<T>
where
    T: Sub<T>,
{
    type Output = Dyn<<T as Sub<T>>::Output>;

    fn sub(self, rhs: Dyn<T>) -> Self::Output {
        Dyn(self.0.sub(rhs.0))
    }
}

impl<T> Mul<Dyn<T>> for Dyn<T>
where
    T: Mul<T>,
{
    type Output = Dyn<<T as Mul<T>>::Output>;

    fn mul(self, rhs: Dyn<T>) -> Self::Output {
        Dyn(self.0.mul(rhs.0))
    }
}

impl<T> Div<Dyn<T>> for Dyn<T>
where
    T: Div<T>,
{
    type Output = Dyn<<T as Div<T>>::Output>;

    fn div(self, rhs: Dyn<T>) -> Self::Output {
        Dyn(self.0.div(rhs.0))
    }
}

typenum_unsigned_impl!(usize, USIZE);
typenum_unsigned_impl!(u8, U8);
typenum_unsigned_impl!(u16, U16);
typenum_unsigned_impl!(u32, U32);
typenum_unsigned_impl!(u64, U64);

typenum_signed_impl!(isize, ISIZE);
typenum_signed_impl!(i8, I8);
typenum_signed_impl!(i16, I16);
typenum_signed_impl!(i32, I32);
typenum_signed_impl!(i64, I64);
