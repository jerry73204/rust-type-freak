use crate::{list, List};

// conversions

macro_rules! ListIdent {
    [] => {
        $crate::list::base::Nil
    };
    [$name:ident $(, $names:ident)* $(,)?] => {
        $crate::list::base::Cons { head: $name, tail: ListIdent![$($names),*] }
    };
}

macro_rules! impl_convert_tuple_list {
    ($($generics:ident),+ ; $($vars:ident),+) => {
        impl<$($generics),*> From<($($generics),* ,)> for List![$($generics),*] {
            fn from(($($vars),* ,): ($($generics),* ,)) -> Self {
                ListIdent![$($vars),*]
            }
        }

        impl<$($generics),*> From<List![$($generics),*]> for ($($generics),* ,) {
            fn from(ListIdent![$($vars),*]: List![$($generics),*]) -> Self {
                ($($vars),* ,)
            }
        }
    }
}

impl From<()> for List![] {
    fn from(_: ()) -> Self {
        list![]
    }
}

impl_convert_tuple_list! (E1; e1);
impl_convert_tuple_list! (E1, E2; e1, e2);
impl_convert_tuple_list! (E1, E2, E3; e1, e2, e3);
impl_convert_tuple_list! (E1, E2, E3, E4; e1, e2, e3, e4);
impl_convert_tuple_list! (E1, E2, E3, E4, E5; e1, e2, e3, e4, e5);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6; e1, e2, e3, e4, e5, e6);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7; e1, e2, e3, e4, e5, e6, e7);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8; e1, e2, e3, e4, e5, e6, e7, e8);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8, E9; e1, e2, e3, e4, e5, e6, e7, e8, e9);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10; e1, e2, e3, e4, e5, e6, e7, e8, e9, e10);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11; e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12; e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13; e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14; e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15; e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15);
impl_convert_tuple_list! (E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16; e1, e2, e3, e4, e5, e6, e7, e8, e9, e10, e11, e12, e13, e14, e15, e16);
