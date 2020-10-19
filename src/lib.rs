#![deny(missing_docs)]

//! This crate provides a `BitSet` type for manipulating bit sets.
//!
//! # Examples
//! 
//! ```
//! use bittyset::{BitSet, bitset};
//!
//! // Create an empty BitSet. We use this turbofish form to make compiler happy,
//! // otherwise we have to write something like `BitSet::<usize>::new()`.
//! let mut set1 = <BitSet>::new();
//! assert!(set1.is_empty());
//!
//! // Insert one element.
//! set1.insert(76);
//!
//! // Make use of the Extend trait.
//! set1.extend(vec![47, 20, 5, 11]);
//!
//! // Remove an element.
//! set1.remove(20);
//! 
//! // Create a BitSet with the convenience macro `bitset!`.
//! let set2 = bitset![5, 12, 47, 104];
//!
//! // Compute the union of two sets.
//! assert_eq!(&set1 | &set2, bitset![5, 11, 12, 47, 76, 104]);
//!
//! // Compute the intersection of two sets.
//! assert_eq!(&set1 & &set2, bitset![5, 47]);
//!
//! // Compute the difference of two sets.
//! assert_eq!(&set1 - &set2, bitset![11, 76]);
//!
//! // Iterate over the set, producing `usize`s.
//! for x in set1.iter() {
//!   println!("{}", x);
//! }
//! ```

pub use self::iter::Iter;
pub use self::block::BitBlock;

mod iter;
mod block;
mod impls;
mod macros;

/// A `BitSet` type based on bit vectors.
///
/// `T` is an unsigned integer type for the underlying bit vector.
#[derive(Clone, Default)]
pub struct BitSet<T = usize> {
  /// # Invariants
  ///
  /// If `num_bits` is not a multiple of `T::NUM_BITS`, the highest
  /// `T::NUM_BITS - num_bits % T::NUM_BITS` bits of the last block of the set
  /// are all zeros.
  ///
  /// the bit indexed by `num_bits - 1` is always set.
  vec: Vec<T>,

  /// Number of all bits (set & unset).
  num_bits: usize,
}

impl<T> BitSet<T>
where
  T: BitBlock,
{
  /// Creates a new empty `BitSet`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::BitSet;
  ///
  /// let set = <BitSet>::new();
  /// assert!(set.is_empty());
  /// ```
  pub fn new() -> Self {
    Self {
      vec: vec![],
      num_bits: 0,
    }
  }

  /// Creates a new empty `BitSet` with the given capacity for the underlying
  /// bit vector.
  ///
  /// Note that the actual capacity of the created `BitSet` may be greater than
  /// the given `capacity`, to make best use of the space of the underlying bit
  /// vector.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::BitSet;
  ///
  /// let set = BitSet::<u8>::with_capacity(5);
  /// assert_eq!(set.capacity(), 8);
  /// ```
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      vec: Vec::with_capacity(compute_num_blocks::<T>(capacity)),
      num_bits: 0,
    }
  }

  /// Returns the capacity of the underlying bit vector.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::BitSet;
  ///
  /// let mut set = BitSet::<u8>::with_capacity(14);
  /// assert_eq!(set.capacity(), 16);
  /// ```
  pub fn capacity(&self) -> usize {
    self.vec.capacity().checked_mul(T::NUM_BITS).unwrap_or(std::usize::MAX)
  }

  /// Reserves capacity for at least `additional` more bits for the underlying bit
  /// vector.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set = bitset![1];
  /// set.reserve(10);
  /// assert!(set.capacity() >= 11);
  /// ```
  pub fn reserve(&mut self, additional: usize) {
    let cap = self.num_bits.checked_add(additional).expect("capacity overflow");
    if cap > self.capacity() {
      let vec_len = self.vec.len();
      self.vec.reserve(compute_num_blocks::<T>(cap) - vec_len);
    }
  }

  /// Reserve capacity for exactly `additional` more bits for the underlying bit
  /// vector.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set = bitset![1];
  /// set.reserve_exact(10);
  /// assert!(set.capacity() >= 11);
  /// ```
  pub fn reserve_exact(&mut self, additional: usize) {
    let cap = self.num_bits.checked_add(additional).expect("capacity overflow");
    if cap > self.capacity() {
      let vec_len = self.vec.len();
      self.vec.reserve_exact(compute_num_blocks::<T>(cap) - vec_len);
    }
  }

  /// Shrinks the capacity of the underlying bit vector as much as possible.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::BitSet;
  ///
  /// let mut set = BitSet::<u8>::with_capacity(30);
  /// set.extend([1, 2, 3].iter().cloned());
  /// assert_eq!(set.capacity(), 32);
  /// set.shrink_to_fit();
  /// assert!(set.capacity() >= 8);
  /// ```
  pub fn shrink_to_fit(&mut self) {
    self.vec.shrink_to_fit();
  }

  fn compact(&mut self) {
    for i in (0..self.vec.len()).rev() {
      let x = self.vec[i];
      if x.count_ones() != 0 {
        self.vec.truncate(i + 1);
        self.num_bits = (i + 1) * T::NUM_BITS - x.leading_zeros() as usize;
        return;
      }
    }
    self.vec.clear();
    self.num_bits = 0;
  }

  /// Iterates over the `BitSet`, producing `usize`s representing the elements
  /// in the set, in ascending order.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let set1 = bitset![7,3,5,18];
  /// let vec1 = set1.iter().collect::<Vec<usize>>();
  ///
  /// assert_eq!(vec1, vec![3,5,7,18]);
  /// ```
  pub fn iter(&self) -> Iter<T> {
    Iter::new(self)
  }

  /// Returns the number of elements in the set.
  pub fn len(&self) -> usize {
    self.vec.iter().map(|x| x.count_ones() as usize).sum()
  }

  /// Returns whether the set is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Clear the set, removing all elements.
  ///
  /// Note that this method has no effect on the allocated capacity of the
  /// underlying bit vector.
  pub fn clear(&mut self) {
    self.vec.clear();
    self.num_bits = 0;
  }

  /// Returns whether the given `value` is present in the set.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set1 = bitset![7,3,5,18];
  ///
  /// assert!(set1.contains(18));
  /// assert!(!set1.contains(4));
  /// ```
  pub fn contains(&self, value: usize) -> bool {
    if value >= self.num_bits {
      return false;
    }

    self.contains_unchecked(value)
  }

  #[inline(always)]
  fn contains_unchecked(&self, value: usize) -> bool {
    self.vec[value / T::NUM_BITS] & (T::one() << (value % T::NUM_BITS)) != T::zero()
  }

  /// Adds a value to the set.
  ///
  /// If the set did not have this value present, `true` is returned.
  ///
  /// If the set did have this value present, `false` is returned.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set1 = bitset![7,3,5,18];
  ///
  /// assert!(set1.insert(13));
  /// assert!(!set1.insert(5));
  /// ```
  pub fn insert(&mut self, value: usize) -> bool {
    let nblks = compute_num_blocks::<T>(value + 1);
    if self.vec.len() < nblks {
      self.vec.resize(nblks, T::zero());
    }

    if self.num_bits < value + 1 {
      self.num_bits = value + 1;
    }

    let present = self.contains_unchecked(value);
    self.vec[value / T::NUM_BITS] |= T::one() << (value % T::NUM_BITS);
    !present
  }

  /// Removes a value from the set. Returns whether the value was present in the set.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set1 = bitset![7,3,5,18];
  ///
  /// assert!(set1.remove(3));
  /// assert!(!set1.remove(13));
  /// ```
  pub fn remove(&mut self, value: usize) -> bool {
    if value >= self.num_bits {
      return false;
    }

    let present = self.contains_unchecked(value);
    self.vec[value / T::NUM_BITS] &= !(T::one() << (value % T::NUM_BITS));

    if present && value + 1 == self.num_bits {
      self.compact();
    }

    present
  }

  /// Computes the union of the set and `other`.
  ///
  /// A corresponding [BitOr](https://doc.rust-lang.org/std/ops/trait.BitOr.html) implementation is also available, i.e. `a | b`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,1,6,7,24];
  /// let set3 = bitset![1,3,5,6,7,18,24];
  ///
  /// assert_eq!(set1.union(&set2), set3);
  /// assert_eq!(set1 | set2, set3);
  /// ```
  pub fn union(&self, other: &Self) -> Self {
    self | other
  }

  /// Computes the union of the set and `other` in place.
  ///
  /// A corresponding [BitOrAssign](https://doc.rust-lang.org/std/ops/trait.BitOrAssign.html) implementation is also available, i.e. `a |= b`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,1,6,7,24];
  /// let set3 = bitset![1,3,5,6,7,18,24];
  ///
  /// set1.union_with(&set2);
  ///
  /// assert_eq!(set1, set3);
  /// ```
  pub fn union_with(&mut self, other: &Self) {
    *self |= other;
  }

  /// Computes the intersection of the set and `other`.
  ///
  /// A corresponding [BitAnd](https://doc.rust-lang.org/std/ops/trait.BitAnd.html) implementation is also available, i.e. `a & b`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,1,6,7,24];
  /// let set3 = bitset![3,7];
  ///
  /// assert_eq!(set1.intersection(&set2), set3);
  /// assert_eq!(set1 & set2, set3);
  /// ```
  pub fn intersection(&self, other: &Self) -> Self {
    self & other
  }

  /// Computes the intersection of the set and `other` in place.
  ///
  /// A corresponding [BitAndAssign](https://doc.rust-lang.org/std/ops/trait.BitAndAssign.html) implementation is also available, i.e. `a &= b`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,1,6,7,24];
  /// let set3 = bitset![3,7];
  ///
  /// set1.intersect_with(&set2);
  ///
  /// assert_eq!(set1, set3);
  /// ```
  pub fn intersect_with(&mut self, other: &Self) {
    *self &= other;
  }

  /// Computes the difference of the set and `other`.
  ///
  /// A corresponding [Sub](https://doc.rust-lang.org/std/ops/trait.Sub.html) implementation is also available, i.e. `a - b`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,1,6,7,24];
  /// let set3 = bitset![5,18];
  ///
  /// assert_eq!(set1.difference(&set2), set3);
  /// assert_eq!(set1 - set2, set3);
  /// ```
  pub fn difference(&self, other: &Self) -> Self {
    self - other
  }

  /// Computes the difference of the set and `other` in place.
  ///
  /// A corresponding [SubAssign](https://doc.rust-lang.org/std/ops/trait.SubAssign.html) implementation is also available, i.e. `a -= b`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,1,6,7,24];
  /// let set3 = bitset![5,18];
  ///
  /// set1.difference_with(&set2);
  ///
  /// assert_eq!(set1, set3);
  /// ```
  pub fn difference_with(&mut self, other: &Self) {
    *self -= other;
  }

  /// Computes the symmetric difference of the set and `other`.
  ///
  /// A corresponding [BitXor](https://doc.rust-lang.org/std/ops/trait.BitXor.html) implementation is also available, i.e. `a ^ b`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,1,6,7,24];
  /// let set3 = bitset![1,5,6,18,24];
  ///
  /// assert_eq!(set1.symmetric_difference(&set2), set3);
  /// assert_eq!(set1 ^ set2, set3);
  /// ```
  pub fn symmetric_difference(&self, other: &Self) -> Self {
    self ^ other
  }

  /// Computes the symmetric difference of the set and `other` in place.
  ///
  /// A corresponding [BitXorAssign](https://doc.rust-lang.org/std/ops/trait.BitXorAssign.html) implementation is also available, i.e. `a ^= b`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let mut set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,1,6,7,24];
  /// let set3 = bitset![1,5,6,18,24];
  ///
  /// set1.symmetric_difference_with(&set2);
  ///
  /// assert_eq!(set1, set3);
  /// ```
  pub fn symmetric_difference_with(&mut self, other: &Self) {
    *self ^= other;
  }

  /// Returns whether the set is a subset of `other`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,5,7,18,41];
  ///
  /// assert!(set1.is_subset(&set2));
  /// assert!(!set2.is_subset(&set1));
  /// ```
  pub fn is_subset(&self, other: &Self) -> bool {
    if self.num_bits > other.num_bits {
      return false;
    }

    let nblks = crate::compute_num_blocks::<T>(self.num_bits);
    for i in 0..nblks {
      if self.vec[i] & !other.vec[i] != T::zero() {
        return false;
      }
    }

    true
  }

  /// Returns whether the set is a proper subset of `other`.
  ///
  /// # Examples
  ///
  /// ```
  /// use bittyset::bitset;
  ///
  /// let set1 = bitset![7,3,5,18];
  /// let set2 = bitset![3,5,7,18,41];
  ///
  /// assert!(set1.is_proper_subset(&set2));
  /// assert!(!set2.is_proper_subset(&set1));
  /// assert!(!set1.is_proper_subset(&set1));
  /// ```
  pub fn is_proper_subset(&self, other: &Self) -> bool {
    if self.num_bits > other.num_bits {
      return false;
    }

    let nblks1 = crate::compute_num_blocks::<T>(self.num_bits);
    let nblks2 = crate::compute_num_blocks::<T>(other.num_bits);
    let mut equal = nblks1 == nblks2;

    for i in 0..nblks1 {
      if self.vec[i] & !other.vec[i] != T::zero() {
        return false;
      }

      if self.vec[i] != other.vec[i] {
        equal = false;
      }
    }

    !equal
  }
}

#[inline(always)]
fn compute_num_blocks<T: BitBlock>(num_bits: usize) -> usize {
  (num_bits + T::NUM_BITS - 1) / T::NUM_BITS
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn insert() {
    let mut set = <BitSet>::new();

    assert_eq!(set.insert(7), true);
    assert_eq!(set.insert(3), true);
    assert_eq!(set.insert(12), true);
    assert_eq!(set.insert(3173), true);
    assert_eq!(set.insert(12), false);

    assert_eq!(set.num_bits, 3174);
  }

  #[test]
  fn remove() {
    let mut set = BitSet::<u8>::new();

    set.insert(7);
    set.insert(3);
    set.insert(12);
    set.insert(173);
    set.insert(12);

    assert_eq!(set.remove(3), true);
    assert_eq!(set.remove(9), false);
    assert_eq!(set.remove(3), false);
    assert_eq!(set.remove(12), true);
    assert_eq!(set.remove(200), false);

    assert_eq!(set.num_bits, 174);

    assert_eq!(set.remove(173), true);

    assert_eq!(set.num_bits, 8);
  }

  #[test]
  fn contains() {
    let mut set = BitSet::<u16>::new();

    set.insert(7);
    set.insert(3);
    set.insert(12);
    set.insert(173);
    set.insert(12);

    assert!(set.contains(12));
    assert!(set.contains(173));
    assert!(!set.contains(200));
    assert!(!set.contains(172));

    set.remove(3);
    set.remove(9);
    set.remove(3);
    set.remove(12);
    set.remove(200);

    assert!(!set.contains(3));
    assert!(set.contains(7));
    assert!(!set.contains(200));
    assert!(set.contains(173));
    assert!(!set.contains(172));
  }

  #[test]
  fn len() {
    let mut set = BitSet::<u64>::new();

    assert_eq!(set.len(), 0);
    assert_eq!(set.num_bits, 0);
    assert!(set.is_empty());

    set.insert(37);
    set.insert(0);
    set.insert(14);
    set.insert(7);
    set.insert(0);

    assert_eq!(set.len(), 4);
    assert_eq!(set.num_bits, 38);
    assert!(!set.is_empty());

    set.remove(7);
    set.remove(14);

    assert_eq!(set.len(), 2);
    assert!(!set.is_empty());

    set.remove(0);
    set.remove(37);

    assert_eq!(set.len(), 0);
    assert_eq!(set.num_bits, 0);
    assert!(set.is_empty());

    set.remove(18);

    assert_eq!(set.num_bits, 0);
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
  }

  #[test]
  fn shrink_to_fit() {
    let mut set = BitSet::<u32>::new();

    set.insert(760);
    set.insert(3173);
    set.shrink_to_fit();

    assert_eq!(set.num_bits, 3174);
    assert_eq!(set.vec.capacity(), 100);
    assert_eq!(set.capacity(), 100 * 32);

    set.insert(63);
    set.remove(3173);
    set.shrink_to_fit();

    assert_eq!(set.num_bits, 761);
    assert_eq!(set.vec.capacity(), 24);
    assert_eq!(set.capacity(), 24 * 32);

    set.remove(760);
    set.shrink_to_fit();

    assert_eq!(set.num_bits, 64);
    assert_eq!(set.vec.capacity(), 2);
    assert_eq!(set.capacity(), 2 * 32);
  }

  #[test]
  fn with_capacity() {
    let set = BitSet::<u16>::with_capacity(60);
    assert_eq!(set.vec.capacity(), 4);

    let set = BitSet::<u64>::with_capacity(6400);
    assert_eq!(set.vec.capacity(), 100);
  }

  #[test]
  fn reserve() {
    let mut set = BitSet::<u16>::new();
    set.insert(33);

    set.reserve(100);

    assert!(set.vec.capacity() >= 9);

    set.reserve(110);

    assert!(set.vec.capacity() >= 9);

    set.reserve_exact(100);

    assert_eq!(set.vec.capacity(), 9);

    set.reserve_exact(110);
  }

}