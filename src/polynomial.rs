use crate::{
    counter::{Counter, Current, Next},
    kvlist::{KVGetOrDefaultValue, KVGetOrDefaultValueOutput, KVList},
};
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};
use typenum::{Sum, P1, Z0};

pub trait Variable {}

pub trait Polynomial {}

pub struct Nomial<Var, Coefs>
where
    Var: Variable,
    Coefs: KVList,
{
    _phantom: PhantomData<(Var, Coefs)>,
}

impl<Var, Coefs> Polynomial for Nomial<Var, Coefs>
where
    Var: Variable,
    Coefs: KVList,
{
}

// TODO
// impl<LVar, LCoefs, RVar, RCoefs> Add<Nomial<RVar, RCoefs>> for Nomial<LVar, LCoefs>
// where
//     LVar: Variable,
//     LCoefs: KVList,
//     RVar: Variable,
//     RCoefs: KVList,
// {
//     type Output =
// }

pub struct ConstantTerm<Value> {
    _phantom: PhantomData<Value>,
}

impl<Value> ConstantTerm<Value> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Value> Polynomial for ConstantTerm<Value> {}

impl<LValue, RValue> Add<ConstantTerm<RValue>> for ConstantTerm<LValue>
where
    LValue: Add<RValue>,
{
    type Output = ConstantTerm<Sum<LValue, RValue>>;

    fn add(self, _rhs: ConstantTerm<RValue>) -> Self::Output {
        Self::Output::new()
    }
}

// TODO
// impl<LValue, RVar, RCoefs> Add<Nomial<RVar, RCoefs>> for ConstantTerm<LValue>
// where
//     RVar: Variable,
//     RCoefs: KVList,
// {
//     type Output = KVGetOrDefaultValueOutput<RCoefs, Z0, ConstantTerm<Z0>, Index>;

//     fn add(self, _rhs: ConstantTerm<RValue>) -> Self::Output {
//         Self::Output::new()
//     }
// }

// major

pub trait ToMajorVariable<Var, Remain>
where
    Var: Variable,
    Remain: Polynomial,
    Self: Polynomial,
    Self::Output: Polynomial,
{
    type Output;
}

impl<Var, Remain, Value> ToMajorVariable<Var, Remain> for ConstantTerm<Value>
where
    Var: Variable,
    Remain: Polynomial,
{
    type Output = Self;
}

impl<Var, Remain, Coefs> ToMajorVariable<Var, Remain> for Nomial<Var, Coefs>
where
    Var: Variable,
    Remain: Polynomial,
    Coefs: KVList,
{
    type Output = Self;
}

pub type ToMajorVariableOutput<Poly, Var, Remain> = <Poly as ToMajorVariable<Var, Remain>>::Output;
