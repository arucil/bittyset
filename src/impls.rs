use std::fmt::{self, Debug, Formatter};
use std::iter::{FromIterator, Extend};
use std::hash::{Hash, Hasher};
use std::ops::{
  BitOr, BitOrAssign, BitAnd, BitAndAssign, Sub, SubAssign, BitXor, BitXorAssign
};
use crate::{BitBlock, BitSet, Iter};

impl<T> Debug for BitSet<T>
where
  T: BitBlock,
{
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    f.debug_set().entries(self).finish()
  }
}

impl<'a, T> IntoIterator for &'a BitSet<T>
where
  T: BitBlock,
{
  type IntoIter = Iter<'a, T>;
  type Item = usize;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<T: BitBlock> FromIterator<usize> for BitSet<T> {
  fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
    let mut set = Self::default();
    set.extend(iter);
    set
  }
}

impl<T: BitBlock> Extend<usize> for BitSet<T> {
  fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
    for x in iter {
      self.insert(x);
    }
  }
}

impl<T: BitBlock> PartialEq<BitSet<T>> for BitSet<T> {
  fn eq(&self, other: &BitSet<T>) -> bool {
    if self.num_bits != other.num_bits {
      return false;
    }

    let nblks = crate::compute_num_blocks::<T>(self.num_bits);
    self.vec[..nblks] == other.vec[..nblks]
  }
}

impl<T: BitBlock> Eq for BitSet<T> {}

impl<T: BitBlock> Hash for BitSet<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    let nblks = crate::compute_num_blocks::<T>(self.num_bits);
    self.vec[..nblks].hash(state);
  }
}

impl<T: BitBlock> BitOr<BitSet<T>> for BitSet<T> {
  type Output = BitSet<T>;

  fn bitor(mut self, rhs: BitSet<T>) -> BitSet<T> {
    self |= rhs;
    self
  }
}

impl<T: BitBlock> BitOrAssign<BitSet<T>> for BitSet<T> {
  fn bitor_assign(&mut self, mut rhs: BitSet<T>) {
    let nblks = crate::compute_num_blocks::<T>(self.num_bits.min(rhs.num_bits));
    if self.num_bits < rhs.num_bits {
      std::mem::swap(self, &mut rhs);
    }

    for i in 0..nblks {
      self.vec[i] |= rhs.vec[i];
    }
  }
}

impl<T: BitBlock> BitAnd<BitSet<T>> for BitSet<T> {
  type Output = BitSet<T>;

  fn bitand(mut self, rhs: BitSet<T>) -> BitSet<T> {
    self &= rhs;
    self
  }
}

impl<T: BitBlock> BitAndAssign<BitSet<T>> for BitSet<T> {
  fn bitand_assign(&mut self, mut rhs: BitSet<T>) {
    let nblks = crate::compute_num_blocks::<T>(self.num_bits.min(rhs.num_bits));
    if self.num_bits > rhs.num_bits {
      std::mem::swap(self, &mut rhs);
    }

    for i in 0..nblks {
      self.vec[i] &= rhs.vec[i];
    }

    self.compact();
  }
}

impl<T: BitBlock> Sub<BitSet<T>> for BitSet<T> {
  type Output = BitSet<T>;

  fn sub(mut self, rhs: BitSet<T>) -> BitSet<T> {
    self -= rhs;
    self
  }
}

impl<T: BitBlock> SubAssign<BitSet<T>> for BitSet<T> {
  fn sub_assign(&mut self, rhs: BitSet<T>) {
    let nblks = crate::compute_num_blocks::<T>(self.num_bits.min(rhs.num_bits));

    for i in 0..nblks {
      self.vec[i] &= !rhs.vec[i];
    }

    self.compact();
  }
}

impl<T: BitBlock> BitXor<BitSet<T>> for BitSet<T> {
  type Output = BitSet<T>;

  fn bitxor(mut self, rhs: BitSet<T>) -> BitSet<T> {
    self ^= rhs;
    self
  }
}

impl<T: BitBlock> BitXorAssign<BitSet<T>> for BitSet<T> {
  fn bitxor_assign(&mut self, mut rhs: BitSet<T>) {
    let nblks = crate::compute_num_blocks::<T>(self.num_bits.min(rhs.num_bits));
    if self.num_bits < rhs.num_bits {
      std::mem::swap(self, &mut rhs);
    }

    for i in 0..nblks {
      self.vec[i] ^= rhs.vec[i];
    }

    self.compact();
  }
}
