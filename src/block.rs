use std::ops::{BitOrAssign, BitAndAssign, BitXorAssign};
use std::hash::Hash;
use num_traits::int::PrimInt;

/// A trait for representing elements of the underlying bit vector of `BitSet`.
pub trait BitBlock: Default + Eq + Hash + PrimInt +
  BitOrAssign + BitAndAssign + BitXorAssign
{
  #[doc(hidden)]
  const NUM_BITS: usize;
}

macro_rules! impl_bit_block {
  ($type:ty) => {
    impl BitBlock for $type {
      const NUM_BITS: usize = std::mem::size_of::<$type>() * 8;
    }
  }
}

impl_bit_block!(u8);
impl_bit_block!(u16);
impl_bit_block!(u32);
impl_bit_block!(u64);
impl_bit_block!(u128);
impl_bit_block!(usize);