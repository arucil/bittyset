use crate::{BitBlock, BitSet};

/// An iterator for `BitSet`.
///
/// This struct is created by the [`iter`] method on [`BitSet`]s.
///
/// [`BitSet`]: struct.BitSet.html
/// [`iter`]: struct.BitSet.html#method.iter
pub struct Iter<'a, T> {
  slice: &'a [T],
  index: usize,
  bit: usize,
}

impl<'a, T> Iter<'a, T> {
  pub(crate) fn new(set: &'a BitSet<T>) -> Self {
    Self {
      slice: &set.vec,
      index: 0,
      bit: 0,
    }
  }
}

impl<'a, T> Iterator for Iter<'a, T>
where
  T: BitBlock,
{
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    while self.index != self.slice.len() {
      if let Some(bit) = self.slice[self.index].find_lowest_set_bit(self.bit) {
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