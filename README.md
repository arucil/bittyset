# bittyset

![crates.io](https://img.shields.io/crates/v/bittyset.svg)
![docs.rs](https://docs.rs/bittyset/badge.svg)

A `BitSet` for manipulating bit sets.

This project aims to replace [bit-set](https://crates.io/crates/bit-set), which is not under active development anymore.

## Goals

- Friendly APIs.
- Performance.

## Installing

Add the following line to your `Cargo.toml`,

```toml
[dependencies]
# ...
bittyset = "0.1.0"
```

[cargo-edit](https://crates.io/crates/cargo-edit) is highly recommended for managing project dependencies. After installing `cargo-edit`, run the following command,
```shell
cargo add bittyset
```

## Examples

```rust
use bittyset::{BitSet, bitset};

// Create an empty BitSet. We use this turbofish form to make compiler happy,
// otherwise we have to write something like `BitSet::<usize>::new()`.
let mut set1 = <BitSet>::new();
assert!(set1.is_empty());

// Insert one element.
set1.insert(76);

// Make use of the Extend trait.
set1.extend(vec![47, 20, 5, 11]);

// Remove an element.
set1.remove(20);

// Create a BitSet with the convenience macro `bitset!`.
let set2 = bitset![5, 12, 47, 104];

// Compute the union of two sets.
assert_eq!(&set1 | &set2, bitset![5, 11, 12, 47, 76, 104]);

// Compute the intersection of two sets.
assert_eq!(&set1 & &set2, bitset![5, 47]);

// Compute the difference of two sets.
assert_eq!(&set1 - &set2, bitset![11, 76]);

// Iterate over the set, producing `usize`s.
for x in set1.iter() {
  println!("{}", x);
}
```