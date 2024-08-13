use crate::{BitBlock, BitSet};

/// An iterator for `BitSet`.
///
/// This struct is created by the [`iter`] method on [`BitSet`]s.
///
/// [`BitSet`]: struct.BitSet.html
/// [`iter`]: struct.BitSet.html#method.iter
pub struct Iter<'a, T> {
  slice: &'a [T],
  num_bits: usize,
  index: usize,
  bit: usize,
}

impl<'a, T> Iter<'a, T> 
where
  T: BitBlock
{
  pub(crate) fn new(set: &'a BitSet<T>) -> Self {
    Self {
      slice: &set.vec,
      num_bits: set.num_bits,
      index: 0,
      bit: 0,
    }
  }
  pub(crate) fn new_from(set: &'a BitSet<T>, start: usize) -> Self {
    let (index, bit) = (start / T::NUM_BITS, start % T::NUM_BITS);
    Self {
      slice: &set.vec,
      num_bits: set.num_bits,
      index,
      bit,
    }
  }
}

impl<'a, T> Iterator for Iter<'a, T>
where
  T: BitBlock,
{
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    while self.index * T::NUM_BITS + self.bit < self.num_bits {
      if let Some(bit) = find_lowest_set_bit(self.slice[self.index], self.bit) {
        self.bit = bit + 1;
        return Some(self.index.checked_mul(T::NUM_BITS)
          .and_then(|x| x.checked_add(bit))
          .expect("element overflow"));
      } else {
        self.index += 1;
        self.bit = 0;
      }
    }
    None
  }
}

fn find_lowest_set_bit<T: BitBlock>(blk: T, from: usize) -> Option<usize> {
  if from >= T::NUM_BITS {
    return None;
  }

  let x = (blk & !((T::one() << from) - T::one())).trailing_zeros() as usize;
  if x == T::NUM_BITS {
    None
  } else {
    Some(x)
  }
}