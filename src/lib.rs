#![deny(missing_docs)]

//! This crate provides a `BitSet` type for manipulating bit sets.

pub use self::iter::Iter;
pub use self::block::BitBlock;

mod iter;
mod block;
mod impls;

///
#[derive(Clone, Default)]
pub struct BitSet<T = usize> {
  /// # Invariant
  /// If `num_bits` is not a multiple of `T::NUM_BITS`, the highest
  /// `T::NUM_BITS - num_bits % T::NUM_BITS` bits of the last block of the set
  /// are all zeros.
  vec: Vec<T>,
  /// Number of all bits (set & unset).
  num_bits: usize,
}

impl<T> BitSet<T>
where
  T: BitBlock,
{
  /// Creates a new empty `BitSet`.
  pub fn new() -> Self {
    Self {
      vec: vec![],
      num_bits: 0,
    }
  }

  /// Creates a new empty `BitSet` with the given capacity for the underlying
  /// bit vector.
  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      vec: Vec::with_capacity(compute_num_blocks::<T>(capacity)),
      num_bits: 0,
    }
  }

  /// Returns the capacity of the underlying bit vector.
  pub fn capacity(&self) -> usize {
    self.vec.capacity().checked_mul(T::NUM_BITS).unwrap_or(std::usize::MAX)
  }

  /// Reserve capacity for at least `additional` more bits for the underlying bit
  /// vector.
  pub fn reserve(&mut self, additional: usize) {
    let cap = self.num_bits.checked_add(additional).expect("capacity overflow");
    if cap > self.capacity() {
      let vec_len = self.vec.len();
      self.vec.reserve(compute_num_blocks::<T>(cap) - vec_len);
    }
  }

  /// Reserve capacity for exactly `additional` more bits for the underlying bit
  /// vector.
  pub fn reserve_exact(&mut self, additional: usize) {
    let cap = self.num_bits.checked_add(additional).expect("capacity overflow");
    if cap > self.capacity() {
      let vec_len = self.vec.len();
      self.vec.reserve_exact(compute_num_blocks::<T>(cap) - vec_len);
    }
  }

  /// Shrinks the capacity of the underlying bit vector as much as possible.
  pub fn shrink_to_fit(&mut self) {
    self.vec.shrink_to_fit();
  }

  fn compact(&mut self) {
    for i in (0..self.vec.len()).rev() {
      let x = self.vec[i];
      if x.count_ones() != 0 {
        self.vec.truncate(i + 1);
        self.num_bits = (i + 1) * T::NUM_BITS - x.highest_zeros();
        return;
      }
    }
    self.vec.clear();
    self.num_bits = 0;
  }

  /// Iterate over the `BitSet`, producing `usize`s.
  pub fn iter(&self) -> Iter<T> {
    Iter::new(self)
  }

  /// Returns the number of elements in the set.
  pub fn len(&self) -> usize {
    self.vec.iter().map(|x| x.count_ones()).sum()
  }

  /// Returns whether the set is empty.
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Removes all elements from the set.
  pub fn clear(&mut self) {
    self.vec.clear();
    self.num_bits = 0;
  }

  /// Returns whether the given `value` is present in the set.
  pub fn contains(&self, value: usize) -> bool {
    if value >= self.num_bits {
      return false;
    }

    self.contains_unchecked(value)
  }

  #[inline(always)]
  fn contains_unchecked(&self, value: usize) -> bool {
    self.vec[value / T::NUM_BITS].bit(value % T::NUM_BITS)
  }

  /// Adds a value to the set. Returns whether the value was present in the set.
  pub fn insert(&mut self, value: usize) -> bool {
    let nblks = compute_num_blocks::<T>(value + 1);
    if self.vec.len() < nblks {
      self.vec.resize(nblks, T::default());
    }

    if self.num_bits < value + 1 {
      self.num_bits = value + 1;
    }

    let present = self.contains_unchecked(value);
    self.vec[value / T::NUM_BITS].set_bit(value % T::NUM_BITS);
    present
  }

  /// Removes a value from the set. Returns whether the value was present in the set.
  pub fn remove(&mut self, value: usize) -> bool {
    if value >= self.num_bits {
      return false;
    }

    let present = self.contains_unchecked(value);
    self.vec[value / T::NUM_BITS].reset_bit(value % T::NUM_BITS);

    if present && value + 1 == self.num_bits {
      self.compact();
    }

    present
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

    assert_eq!(set.insert(7), false);
    assert_eq!(set.insert(3), false);
    assert_eq!(set.insert(12), false);
    assert_eq!(set.insert(3173), false);
    assert_eq!(set.insert(12), true);

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