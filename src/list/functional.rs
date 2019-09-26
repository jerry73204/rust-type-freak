// functor and fmap for list
// currently not working

// pub trait Functor {
//     type Output<In>;
// }

// pub trait LMap<Func>
// where
//     Self: TList,
//     Func: Functor,
// {
//     type Output;
// }

// pub type LMapOutput<List, Func> = <List as LMap<Func>>::Output;

// impl<Func> LMap<Func> for LNil
// where
//     Func: Functor,
// {
//     type Output = LNil;
// }

// impl<Func, Head, Tail> LMap<Func> for LCons<Head, Tail>
// where
//     Func: Functor,
//     Tail: TList + LMap<Func>,
//     LMapOutput<Tail, Func>: TList,
// {
//     type Output = LCons<Func::Output<Head>, LMapOutput<Tail, Func>>;
// }
