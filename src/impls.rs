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

macro_rules! op_impl_assign_body {
  ( $self:ident, $rhs:ident, $fn:ident, $swap_cond:tt ) => {{
    if $self.num_bits $swap_cond $rhs.num_bits {
      std::mem::swap($self, &mut $rhs);
    }
    $fn($self, &$rhs.vec, $rhs.num_bits);
  }};
  ( $self:ident, $rhs:ident, $fn:ident ) => {{
    $fn($self, &$rhs.vec, $rhs.num_bits);
  }};
}

macro_rules! op_impl_ref_body {
  ( $self:ident, $rhs:ident, $fn:ident, $swap_cond:tt ) => {{
    if $self.num_bits $swap_cond $rhs.num_bits {
      let mut lhs = $rhs.clone();
      $fn(&mut lhs, &$self.vec, $self.num_bits);
      lhs
    } else {
      let mut lhs = $self.clone();
      $fn(&mut lhs, &$rhs.vec, $rhs.num_bits);
      lhs
    }
  }};
  ( $self:ident, $rhs:ident, $fn:ident ) => {{
    let mut lhs = $self.clone();
    $fn(&mut lhs, &$rhs.vec, $rhs.num_bits);
    lhs
  }};
}

macro_rules! op_impl_assign_ref_body {
  ( $self:ident, $rhs:ident, $fn:ident, $swap_cond:tt ) => {{
    if $self.num_bits $swap_cond $rhs.num_bits {
      let rhs_nbits = $self.num_bits;
      $self.num_bits = $rhs.num_bits;
      if 0 $swap_cond 1 {
        $self.vec.extend_from_slice(&$rhs.vec[$self.vec.len()..]);
      } else {
        $self.vec.truncate($rhs.vec.len());
      }
      $fn($self, &$rhs.vec, rhs_nbits)
    } else {
      $fn($self, &$rhs.vec, $rhs.num_bits)
    }
  }};
  ( $self:ident, $rhs:ident, $fn:ident ) => {{
    $fn($self, &$rhs.vec, $rhs.num_bits)
  }};
}

macro_rules! op_impl {
  ( $fn:ident , ($name:ident, $method:ident, $assign_op:tt),
    ($assign_name:ident, $assign_method:ident),
    ( $lhs:ident, $rhs_vec:ident, $rhs_nbits:ident $(, $swap_cond:tt)? )
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

      fn $method(self, rhs: &'a BitSet<T>) -> BitSet<T> {
        op_impl_ref_body!{ self, rhs, $fn $(, $swap_cond)?}
      }
    }

    impl<T: BitBlock> $assign_name<BitSet<T>> for BitSet<T> {
      #[allow(unused)]
      fn $assign_method(&mut self, mut rhs: BitSet<T>) {
        op_impl_assign_body!{ self, rhs, $fn $(, $swap_cond)?};
      }
    }

    impl<'a, T: BitBlock> $assign_name<&'a BitSet<T>> for BitSet<T> {
      fn $assign_method(&mut self, rhs: &'a BitSet<T>) {
        op_impl_assign_ref_body!{ self, rhs, $fn $(, $swap_cond)?};
      }
    }

    #[inline(always)]
    fn $fn<T: BitBlock>(
      $lhs: &mut BitSet<T>,
      $rhs_vec: &[T],
      $rhs_nbits: usize
    ) {
      $body
    }
  };

  ( op = ($name:ident, $method:ident, $assign_op:tt),
    $(swap_cond = $swap_cond:tt ,)?
    op_assign = ($assign_name:ident, $assign_method:ident),
    ( $lhs:ident, $rhs_vec:ident, $rhs_nbits:ident )
    $body:expr
  ) => {
    gensym::gensym!{
      op_impl!{
        ($name, $method, $assign_op),
        ($assign_name, $assign_method),
        ($lhs, $rhs_vec, $rhs_nbits $(, $swap_cond)?)
        $body
      }
    }
  }
}

op_impl!{
  op = (BitOr, bitor, |=),
  swap_cond = <,
  op_assign = (BitOrAssign, bitor_assign),
  (lhs, rhs_vec, rhs_nbits) {
    let nblks = crate::compute_num_blocks::<T>(lhs.num_bits.min(rhs_nbits));

    for i in 0..nblks {
      lhs.vec[i] |= rhs_vec[i];
    }
  }
}

op_impl!{
  op = (BitAnd, bitand, &=),
  swap_cond = >,
  op_assign = (BitAndAssign, bitand_assign),
  (lhs, rhs_vec, rhs_nbits) {
    let nblks = crate::compute_num_blocks::<T>(lhs.num_bits.min(rhs_nbits));

    for i in 0..nblks {
      lhs.vec[i] &= rhs_vec[i];
    }

    lhs.compact();
  }
}

op_impl!{
  op = (BitXor, bitxor, ^=),
  swap_cond = <,
  op_assign = (BitXorAssign, bitxor_assign),
  (lhs, rhs_vec, rhs_nbits) {
    let nblks = crate::compute_num_blocks::<T>(lhs.num_bits.min(rhs_nbits));

    for i in 0..nblks {
      lhs.vec[i] ^= rhs_vec[i];
    }

    lhs.compact();
  }
}

op_impl!{
  op = (Sub, sub, -=),
  op_assign = (SubAssign, sub_assign),
  (lhs, rhs_vec, rhs_nbits) {
    let nblks = crate::compute_num_blocks::<T>(lhs.num_bits.min(rhs_nbits));

    for i in 0..nblks {
      lhs.vec[i] &= !rhs_vec[i];
    }

    lhs.compact();
  }
}