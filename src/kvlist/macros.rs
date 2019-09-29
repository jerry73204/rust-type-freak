/// Builds a type that implements [KVList](crate::kvlist::KVList).
///
/// ```rust
/// use type_freak::KVListType;
/// use typenum::consts::*;
/// type List = KVListType![(U0, String), (U3, usize)];
/// // Same as KVCons<U0, String, KVCons<U3, usize, KVNil>>
/// ```
#[macro_export]
macro_rules! KVListType {
    () => { $crate::kvlist::KVNil };
    (($name:ty, $value:ty)) => { $crate::kvlist::KVCons<$name, $value, $crate::kvlist::KVNil> };
    (($name:ty, $value:ty), $(($names:ty, $values:ty)),+) => { $crate::kvlist::KVCons<$name, $value, $crate::KVListType![$(($names, $values)),*]> };
}
