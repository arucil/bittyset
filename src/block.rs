use std::ops::{BitOrAssign, BitAndAssign, BitXorAssign};
use std::hash::Hash;
use num_traits::int::PrimInt;

/// A trait for representing elements of the underlying vector of `BitSet`.
pub trait BitBlock: Default + Eq + Hash + PrimInt +
  BitOrAssign + BitAndAssign + BitXorAssign
{
  #[doc(hidden)]
  const NUM_BITS: usize;

  #[doc(hidden)]
  fn find_lowest_set_bit(self, from: usize) -> Option<usize>;

  #[doc(hidden)]
  fn bit(self, index: usize) -> bool;

  #[doc(hidden)]
  fn set_bit(&mut self, index: usize);

  #[doc(hidden)]
  fn reset_bit(&mut self, index: usize);

  #[doc(hidden)]
  fn lowest_bits_eq(self, other: Self, nbits: usize) -> bool;

  #[doc(hidden)]
  fn hash_lowest_bits<H: std::hash::Hasher>(self, nbits: usize, state: &mut H);
}

macro_rules! impl_bit_block {
  ($type:ty) => {
    impl BitBlock for $type {
      const NUM_BITS: usize = std::mem::size_of::<$type>() * 8;

      fn find_lowest_set_bit(self, from: usize) -> Option<usize> {
        if from >= Self::NUM_BITS {
          return None;
        }

        let x = (self & !((1 << from) - 1)).trailing_zeros() as usize;
        if x == Self::NUM_BITS {
          None
        } else {
          Some(x)
        }
      }

      fn bit(self, index: usize) -> bool {
        self & (1 << index) != 0
      }

      fn set_bit(&mut self, index: usize) {
        assert!(index < Self::NUM_BITS,
          "index out of range: {} >= {}", index, Self::NUM_BITS);
        *self |= 1 << index;
      }

      fn reset_bit(&mut self, index: usize) {
        assert!(index < Self::NUM_BITS,
          "index out of range: {} >= {}", index, Self::NUM_BITS);
        *self &= !(1 << index);
      }

      fn lowest_bits_eq(self, other: $type, nbits: usize) -> bool {
        if nbits >= Self::NUM_BITS {
          self == other
        } else {
          self & ((1 << nbits) - 1) == other & ((1 << nbits) - 1)
        }
      }

      fn hash_lowest_bits<H: std::hash::Hasher>(self, nbits: usize, state: &mut H) {
        use std::hash::Hash;

        if nbits >= Self::NUM_BITS {
          self.hash(state);
        } else {
          (self & ((1 << nbits) - 1)).hash(state)
        }
      }
    }
  }
}

impl_bit_block!(u8);
impl_bit_block!(u16);
impl_bit_block!(u32);
impl_bit_block!(u64);
impl_bit_block!(u128);
impl_bit_block!(usize);