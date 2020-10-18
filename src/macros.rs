
/// Creates a `BitSet` containing the arguments.
///
/// # Examples
///
/// ```
/// use bittyset::bitset;
///
/// let set = bitset![4,10,2,7];
/// assert_eq!(set.len(), 4);
/// assert_eq!(format!("{:?}", set), "{2, 4, 7, 10}");
///
/// // The generic parameter of BitSet can be specified explicitly.
/// let set2 = bitset![usize; 2,7,10,4,7];
/// assert_eq!(set, set2);
/// ```
#[macro_export]
macro_rules! bitset {
  ($t:ty $(;)?) => {
    let mut set = <$crate::BitSet::<$t>>::new();
    $(
      set.insert($elem);
    )*
    set
  };
  ($t:ty ; $($elem:expr),* $(,)?) => {{
    let mut set = <$crate::BitSet>::new();
    $(
      set.insert($elem);
    )*
    set
  }};
  () => {
    <$crate::BitSet>::new()
  };
  ($($elem:expr),+ $(,)?) => {{
    let mut set = <$crate::BitSet>::new();
    $(
      set.insert($elem);
    )*
    set
  }};
}