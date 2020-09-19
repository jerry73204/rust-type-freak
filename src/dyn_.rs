use crate::common::*;

// macros

macro_rules! typenum_unsigned_impl {
    ($ty:ty, $output:ident) => {
        impl Add<UTerm> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: UTerm) -> Self::Output {
                Dyn(self.0 + UTerm::$output)
            }
        }

        impl<U, B> Add<UInt<U, B>> for Dyn<$ty>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: UInt<U, B>) -> Self::Output {
                Dyn(self.0 + UInt::<U, B>::$output)
            }
        }

        impl Sub<UTerm> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: UTerm) -> Self::Output {
                Dyn(self.0 - UTerm::$output)
            }
        }

        impl<U, B> Sub<UInt<U, B>> for Dyn<$ty>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: UInt<U, B>) -> Self::Output {
                Dyn(self.0 - UInt::<U, B>::$output)
            }
        }

        impl Mul<UTerm> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: UTerm) -> Self::Output {
                Dyn(self.0 * UTerm::$output)
            }
        }

        impl<U, B> Mul<UInt<U, B>> for Dyn<$ty>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: UInt<U, B>) -> Self::Output {
                Dyn(self.0 * UInt::<U, B>::$output)
            }
        }

        impl<U, B> Div<UInt<U, B>> for Dyn<$ty>
        where
            U: Unsigned,
            B: Bit,
        {
            type Output = Dyn<$ty>;

            fn div(self, _rhs: UInt<U, B>) -> Self::Output {
                Dyn(self.0 / UInt::<U, B>::$output)
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
                Dyn(self.0 + PInt::<U>::$output)
            }
        }

        impl<U> Add<NInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: NInt<U>) -> Self::Output {
                Dyn(self.0 + NInt::<U>::$output)
            }
        }

        impl Add<Z0> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn add(self, _rhs: Z0) -> Self::Output {
                Dyn(self.0 + Z0::$output)
            }
        }

        impl<U> Sub<PInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: PInt<U>) -> Self::Output {
                Dyn(self.0 - PInt::<U>::$output)
            }
        }

        impl<U> Sub<NInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: NInt<U>) -> Self::Output {
                Dyn(self.0 - NInt::<U>::$output)
            }
        }

        impl Sub<Z0> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn sub(self, _rhs: Z0) -> Self::Output {
                Dyn(self.0 - Z0::$output)
            }
        }

        impl<U> Mul<PInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: PInt<U>) -> Self::Output {
                Dyn(self.0 * PInt::<U>::$output)
            }
        }

        impl<U> Mul<NInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: NInt<U>) -> Self::Output {
                Dyn(self.0 * NInt::<U>::$output)
            }
        }

        impl Mul<Z0> for Dyn<$ty> {
            type Output = Dyn<$ty>;

            fn mul(self, _rhs: Z0) -> Self::Output {
                Dyn(self.0 * Z0::$output)
            }
        }

        impl<U> Div<PInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn div(self, _rhs: PInt<U>) -> Self::Output {
                Dyn(self.0 / PInt::<U>::$output)
            }
        }

        impl<U> Div<NInt<U>> for Dyn<$ty>
        where
            U: Unsigned + NonZero,
        {
            type Output = Dyn<$ty>;

            fn div(self, _rhs: NInt<U>) -> Self::Output {
                Dyn(self.0 / NInt::<U>::$output)
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
        Dyn(self.0 + rhs.0)
    }
}

impl<T> Sub<Dyn<T>> for Dyn<T>
where
    T: Sub<T>,
{
    type Output = Dyn<<T as Sub<T>>::Output>;

    fn sub(self, rhs: Dyn<T>) -> Self::Output {
        Dyn(self.0 - rhs.0)
    }
}

impl<T> Mul<Dyn<T>> for Dyn<T>
where
    T: Mul<T>,
{
    type Output = Dyn<<T as Mul<T>>::Output>;

    fn mul(self, rhs: Dyn<T>) -> Self::Output {
        Dyn(self.0 * rhs.0)
    }
}

impl<T> Div<Dyn<T>> for Dyn<T>
where
    T: Div<T>,
{
    type Output = Dyn<<T as Div<T>>::Output>;

    fn div(self, rhs: Dyn<T>) -> Self::Output {
        Dyn(self.0 / rhs.0)
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
