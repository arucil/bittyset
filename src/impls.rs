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

macro_rules! op_impl {
  ( $fn:ident ,; ($name:ident, $method:ident, $assign_op:tt),
    ($assign_name:ident, $assign_method:ident),
    ( $lhs:ident, $rhs:ident $(, $swap_cond:tt)? ),
    $body:tt
  ) => {
    impl<T: BitBlock> $name<BitSet<T>> for BitSet<T> {
      type Output = BitSet<T>;

      fn $method(mut self, rhs: BitSet<T>) -> BitSet<T> {
        self $assign_op rhs;
        self
      }
    }

    impl<'a, T: BitBlock> $name<&'a BitSet<T>> for &'a BitSet<T> {
      type Output = BitSet<T>;

      #[allow(unused)]
      fn $method(self, mut rhs: &'a BitSet<T>) -> BitSet<T> {
        let mut lhs = self;
        $(
          if lhs.num_bits $swap_cond rhs.num_bits {
            std::mem::swap(&mut lhs, &mut rhs);
          }
        )?
        let mut lhs = lhs.clone();
        $fn(&mut lhs, rhs);
        lhs
      }
    }

    impl<T: BitBlock> $assign_name<BitSet<T>> for BitSet<T> {
      #[allow(unused)]
      fn $assign_method(&mut self, mut rhs: BitSet<T>) {
        $(
          if self.num_bits $swap_cond rhs.num_bits {
            std::mem::swap(self, &mut rhs);
          }
        )?
        $fn(self, &rhs)
      }
    }

    #[inline(always)]
    fn $fn<T: BitBlock>($lhs: &mut BitSet<T>, $rhs: &BitSet<T>) {
      $body
    }
  };
  ( ($name:ident, $method:ident, $assign_op:tt),
    ($assign_name:ident, $assign_method:ident),
    ( $lhs:ident, $rhs:ident $(, $swap_cond:tt)?),
    $body:tt
  ) => {
    gensym::gensym!{
      op_impl!{
        ; ($name, $method, $assign_op),
        ($assign_name, $assign_method),
        ($lhs, $rhs $(, $swap_cond)?),
        $body
      }
    }
  }
}

op_impl!((BitOr, bitor, |=), (BitOrAssign, bitor_assign), (lhs, rhs, <), {
  let nblks = crate::compute_num_blocks::<T>(lhs.num_bits.min(rhs.num_bits));

  for i in 0..nblks {
    lhs.vec[i] |= rhs.vec[i];
  }
});

op_impl!((BitAnd, bitand, &=), (BitAndAssign, bitand_assign), (lhs, rhs, >), {
  let nblks = crate::compute_num_blocks::<T>(lhs.num_bits.min(rhs.num_bits));

  for i in 0..nblks {
    lhs.vec[i] &= rhs.vec[i];
  }

  lhs.compact();
});

op_impl!((BitXor, bitxor, ^=), (BitXorAssign, bitxor_assign), (lhs, rhs, <), {
  let nblks = crate::compute_num_blocks::<T>(lhs.num_bits.min(rhs.num_bits));

  for i in 0..nblks {
    lhs.vec[i] ^= rhs.vec[i];
  }

  lhs.compact();
});

op_impl!((Sub, sub, -=), (SubAssign, sub_assign), (lhs, rhs), {
  let nblks = crate::compute_num_blocks::<T>(lhs.num_bits.min(rhs.num_bits));

  for i in 0..nblks {
    lhs.vec[i] &= !rhs.vec[i];
  }

  lhs.compact();
});