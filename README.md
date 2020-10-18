# bittyset

[![crates.io](https://img.shields.io/crates/v/bittyset.svg)](https://crates.io/crates/bittyset)
[![docs.rs](https://docs.rs/bittyset/badge.svg)](https://docs.rs/bittyset)

A `BitSet` for manipulating bit sets.

This project aims to replace [bit-set](https://crates.io/crates/bit-set), which is not under active development anymore.

## Goals

- Friendly APIs.
- Performance.
<details>
  <summary>
    Performance comparison with <a href="https://crates.io/crates/bit-set">bit-set</a>
  </summary>

```
test insert/bit_set ... bench:       55976 ns/iter (+/- 2456)
test insert/bittyset ... bench:       19206 ns/iter (+/- 1622)

test remove/bit_set ... bench:       26914 ns/iter (+/- 975)
test remove/bittyset ... bench:        9013 ns/iter (+/- 359)

test contains/bit_set ... bench:        8498 ns/iter (+/- 175)
test contains/bittyset ... bench:        4200 ns/iter (+/- 34)

test union/bit_set ... bench:        2897 ns/iter (+/- 483)
test union/bittyset ... bench:        1670 ns/iter (+/- 272)

test intersection/bit_set ... bench:        2654 ns/iter (+/- 130)
test intersection/bittyset ... bench:        1968 ns/iter (+/- 188)

test difference/bit_set ... bench:        2835 ns/iter (+/- 226)
test difference/bittyset ... bench:        1712 ns/iter (+/- 254)

test symmetric_difference/bit_set ... bench:        2616 ns/iter (+/- 131)
test symmetric_difference/bittyset ... bench:        1956 ns/iter (+/- 188)

test is_subset/bit_set ... bench:          88 ns/iter (+/- 19)
test is_subset/bittyset ... bench:          87 ns/iter (+/- 21)

test is_subset(self)/bit_set ... bench:        1344 ns/iter (+/- 95)
test is_subset(self)/bittyset ... bench:          33 ns/iter (+/- 10)
```
</details>

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