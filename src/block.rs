
/// A trait for representing elements of the underlying vector of `BitSet`.
pub trait BitBlock: Copy + Default {
  #[doc(hidden)]
  const NUM_BITS: usize;

  #[doc(hidden)]
  fn find_lowest_set_bit(self, from: usize) -> Option<usize>;

  #[doc(hidden)]
  fn highest_zeros(self) -> usize;

  #[doc(hidden)]
  fn count_ones(self) -> usize;

  #[doc(hidden)]
  fn bit(self, index: usize) -> bool;

  #[doc(hidden)]
  fn set_bit(&mut self, index: usize);

  #[doc(hidden)]
  fn reset_bit(&mut self, index: usize);
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

      fn count_ones(self) -> usize {
        self.count_ones() as usize
      }

      fn highest_zeros(self) -> usize {
        self.leading_zeros() as usize
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
    }
  }
}

impl_bit_block!(u8);
impl_bit_block!(u16);
impl_bit_block!(u32);
impl_bit_block!(u64);
impl_bit_block!(u128);
impl_bit_block!(usize);