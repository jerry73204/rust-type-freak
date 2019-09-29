# Type Freak

The crate is a collection of typed data structures, trait operators and
useful type aliases for Rust.
It was introduced to support [tch-typed-tensor](https://github.com/jerry73204/tch-typed-tensor) project,
which provides compile-time checked tensor type.

It reduces runtime computation to minimum by design.
The DSTs are manipulated by _trait operators_.
That is, with Rust's associated types and generics,
we can build non-trivial types like lists and key-value map.

So far, the crate ships following features. It's still in alpha stage and I'm glad for contributions!

- [TList](src/list/mod.rs): a typed list with arbitrary type as keys.
- [KVList](src/kvlist/mod.rs): like [TList](src/list/mod.rs), with extra values.
- [Boolean](src/boolean.rs): typed boolean algebra.
- [Maybe](src/maybe.rs): a trait analogous to `std::option::Option`.
- [Counter](src/counter.rs): a convient type to build recursive trait operators.
- [Functoinal primitives](src/functional/mod.rs): provides `Functor`, `Compose` and `Applicative`, etc.
- [Trait operators for tuple types](src/tuple.rs)
- [Control flow](src/control.rs): typed `If`, `IfLess`, `IfSame` and more for compile-time guards and static assertions.

## Usage

Put this line to your `Cargo.toml`. Note that the crate is still in alpha stage.
Stabilized API is not guaranteed.

```toml
type-freak = "~0"
```

## Examples

### Compile-time guards and static assertions

To assert one typed integer is less than the other typed integer:

```rust
use typenum::consts::*;
use type_freak::control::IfLessOutput;

type Out1 = IfLessOutput<usize, U3, U5>;  // U3 < U5 is true, thus Out1 ~= usize
type Out2 = IfLessOutput<usize, U5, U3>;  // U5 < U5 is false

fn assert() {
   let _: Out1 = 0;  // Goes fine here.
   let _: Out2 = 0;  // Compile error!!!
}
 ```

We can make sure two generic parameters are of the same type by `IfSame`
trait bound.

```rust
use type_freak::control::IfSame;

fn guarded_function<Lhs, Rhs>() -> String
where
    Lhs: IfSame<Lhs, Rhs>
{
    "Yeeeeeee!".to_owned()
}

fn comile_me() {
    let _ = guarded_function::<String, String>();  // fine
    let _ = guarded_function::<String, u8>();      // Compile error!!!
}
```

### Typed list

The `TList` type represents a list of arbitrary types. It can be constructed
by `TListType!` macro. The crate ships a variety of traits as _type operators_ to
manipuate the list structure.

```rust
use type_freak::{TListType, list::*};

type List1 = TListType![u8, u16, u32];

type List2 = LPrepend<List1, u64>;
// List2 ~= TListType![u64, u8, u16, u32]

type List3<Index1> = LRemoveAt<List2, u16, Index1>;
// List3<_> ~= TListType![u64, u8, u32]

type List4<Index1> = LAppend<List3<Index1>, f32>;
// List4 ~= TListType![u64, u8, u32, f32]

type List5<Index1, Index2> = LInsertAt<List4<Index1>, u8, f64, Index2>;
// List5 ~= TListType![u64, u8, f64, u32, f32]
```

### Functional interface

You can map, filter or scan a `TList` with existing functors in crate.
Also, it's allowed to roll your own functor to manipulate the data with ease.

```rust
struct BoxFunctor;

impl<Input> Functor<Input> for BoxFunctor {
    type Output = Box<Input>;
}

type ListBefore = TListType![String, [i64; 7], isize, (), (f64, f32)];
type ListAfter = LMap<List3, BoxFunctor>;

type Assert = IfSameOutput<
    (),
    ListAfter,
    TListType! {
        Box<String>,
        Box<[i64; 7]>,
        Box<isize>,
        Box<()>,
        Box<(f64, f32)>
    },
>;

fn assert() {
    let _: Assert = ();  // static assertion
}
```

### Trait-level `Option`

The `Maybe` is analogous to std's `Option`.

```rust
use typenum::consts::*;
use type_freak::maybe::{Maybe, Just, Nothing};

type Opt1 = Just<U3>;
type Opt2 = Nothing;

type Val1 = Unwrap<Opt1>;       // U3
type Val2 = UnwrapOr<Opt1, U0>; // U3
type Val3 = UnwrapOr<Opt2, U0>; // U0
```

### Auto-inferred counters

The `Counter` traits along with `Next` and `Current` types are handly
tools to build recursive type operators. The following demo implements
an trait that removes a specific type from `TList`.

The example works by a termination step and recursive step, corresponding
to to impl blocks. Note that the `Index` argument is necessary to let compiler
distinguish the signatures of two impl blocks. Otherwise, the compiler will
complain about conflicting implementations.


```rust
use type_freak::{
    list::{TList, LCons, LNil},
    counter::{Counter, Current, Next},
};

/* Definition */

pub trait LRemoveAt<Target, Index>
where
    Index: Counter,
    Self: TList,
    Self::Output: TList,
{
    type Output;
}

// termination step
impl<Target, Tail> LRemoveAt<Target, Current> for LCons<Target, Tail>
where
    Tail: TList,
{
    type Output = Tail;
}

// recursion step
impl<Target, Index, NonTarget, Tail> LRemoveAt<Target, Next<Index>> for LCons<NonTarget, Tail>
where
    Index: Counter,
    Tail: TList + LRemoveAt<Target, Index>,
{
    type Output = LCons<NonTarget, <Tail as LRemoveAt<Target, Index>>::Output>;
}

/* Auto-inference example */

// Here SomeList is equivalent to TListType![u8, u32]
type SomeList<Index> = <TListType![u8, u16, u32] as LRemoveAt<u16, Index>>::Output;

// The Index argument can be inferred by compiler
fn auto_inference() {
    let _ = SomeList::<_>::new();
}
```


## License

The project licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE). Pick the one that suits you.
