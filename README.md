# Type Freak

The crate is a collection of typed data structures, trait operators and useful type aliases for Rust. It was introduced to support [tch-typed-tensor](https://github.com/jerry73204/tch-typed-tensor) project, which provides compile-time checked tensor type.

It reduces runtime computation to minimum by design. The DSTs are manipulated by _trait operators_. That is, with Rust's associated types and generics, we can build non-trivial types like lists and key-value map.

So far, the crate ships following features. It's still in alpha stage and I'm glad for contributions!

- [TList](src/list/mod.rs): a typed list with arbitrary type as keys.
- [KVList](src/kvlist.rs): like [TList](src/list/mod.rs), with extra values.
- [Boolean](src/boolean.rs): typed boolean algebra.
- [Maybe](src/maybe.rs): a trait analogous to `std::optoin::Option`.
- [Trait operators for tuple types](src/tuple.rs)
- [Counter](src/counter.rs): a convient type to build recursive trait operators.
- [Control flow](src/control.rs): typed `If`, used to build compile-time guards.

## Usage

It's not published on crates.io yet. To give it a try, put this on your `Cargo.toml`.

```toml
type-freak = { git = "https://github.com/jerry73204/rust-type-freak.git", branch = "master" }
```

## License

[MIT License](LICENSE)
