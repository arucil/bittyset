use bittyset::{BitSet, bitset};
use pretty_assertions::{assert_eq, assert_ne};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use quickcheck_macros::quickcheck;

mod helper;

#[test]
fn clear() {
  let mut set = <BitSet>::new();

  set.insert(37);
  set.insert(0);
  set.insert(14);
  set.insert(7);
  set.insert(0);

  assert_eq!(set.len(), 4);
  assert!(!set.is_empty());

  set.clear();

  assert_eq!(set.len(), 0);
  assert!(set.is_empty());
}

#[test]
fn debug() {
  let mut set = <BitSet>::new();

  assert_eq!(&format!("{:?}", set), "{}");

  set.insert(37);
  set.insert(0);
  set.insert(14);
  set.insert(7);
  set.insert(0);

  assert_eq!(&format!("{:?}", set), "{0, 7, 14, 37}");

  set.remove(7);

  assert_eq!(&format!("{:?}", set), "{0, 14, 37}");

  set.clear();

  assert_eq!(&format!("{:?}", set), "{}");
}

#[test]
fn iter() {
  let mut set = <BitSet>::new();

  set.insert(37);
  set.insert(0);
  set.insert(14);
  set.insert(7);
  set.insert(0);

  assert_eq!(set.iter().collect::<Vec<_>>(), vec![0,7,14,37]);
}

#[test]
fn extend() {
  let mut set = <BitSet>::new();
  set.extend(vec![37,0,14,7,14]);

  assert_eq!(set.iter().collect::<Vec<_>>(), vec![0,7,14,37]);
}

#[test]
fn collect() {
  let set = bitset![37,0,14,7,14];

  assert_eq!(set.iter().collect::<Vec<_>>(), vec![0,7,14,37]);
}

#[test]
fn eq() {
  let set1 = bitset![7,1,4,5,41,4];
  let mut set2 = bitset![7,1,41,4];

  assert_ne!(set1, set2);

  set2.insert(5);

  assert_eq!(set1, set2);

  set2.remove(41);

  assert_ne!(set1, set2);

  assert_eq!(<BitSet>::new(), <BitSet>::new());

  let set1 = bitset![63];
  let set2 = bitset![63];

  assert_eq!(set1, set2);
}

#[test]
fn eq_large() {
  let set1 = (0..1485914).step_by(4).collect::<BitSet>();
  let mut set2 = set1.clone();

  assert_eq!(set1, set2);

  assert!(set2.remove(1385912));

  assert_ne!(set1, set2);

  set2.insert(1385912);
  set2.remove(1385912 - 4 * 50);

  assert_ne!(set1, set2);
}

fn my_hash<T>(obj: T) -> u64
where
  T: Hash,
{
  let mut hasher = DefaultHasher::new();
  obj.hash(&mut hasher);
  hasher.finish()
}

#[test]
fn hash() {
  let set1 = bitset![7,1,4,5,41,4];
  let mut set2 = bitset![7,1,41,4];

  assert_ne!(my_hash(&set1), my_hash(&set2));

  set2.insert(5);

  assert_eq!(my_hash(&set1), my_hash(&set2));

  set2.remove(41);

  assert_ne!(my_hash(&set1), my_hash(&set2));

  assert_eq!(<BitSet>::new(), <BitSet>::new());

  let set1 = bitset![63];
  let set2 = bitset![63];

  assert_eq!(my_hash(&set1), my_hash(&set2));
}

#[test]
fn hash_large() {
  let set1 = (0..1485914).step_by(4).collect::<BitSet>();
  let mut set2 = set1.clone();

  assert_eq!(my_hash(&set1), my_hash(&set2));

  assert!(set2.remove(1385912));

  assert_ne!(my_hash(&set1), my_hash(&set2));

  set2.insert(1385912);
  set2.remove(1385912 - 4 * 50);

  assert_ne!(my_hash(&set1), my_hash(&set2));
}

#[quickcheck]
fn bitor_prop(vec1: Vec<u16>, vec2: Vec<u16>) -> bool {
  let vec1 = vec1.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let vec2 = vec2.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let set1 = vec1.clone().into_iter().collect::<BitSet>();
  let set2 = vec2.clone().into_iter().collect::<BitSet>();
  let mut vec1 = vec1;
  vec1.extend(vec2);
  let set3 = vec1.into_iter().collect::<HashSet<_>>().into_iter().collect::<BitSet>();

  &set1 | &set2 == set3 && set1 | set2 == set3
}

#[test]
fn bitor_large() {
  let set1 = (0..1000000).step_by(5).collect::<BitSet>();
  let set2 = (0..1000000).step_by(3).collect::<BitSet>();
  let mut set3 = set1.clone();
  set3.extend((0..1000000).step_by(3));

  assert_eq!(set1.clone() | set2.clone(), set3);

  assert_eq!(set2 | set1, set3);
}

#[quickcheck]
fn bitand_prop(vec1: Vec<u16>, vec2: Vec<u16>) -> bool {
  let vec1 = vec1.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let vec2 = vec2.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let set1 = vec1.clone().into_iter().collect::<BitSet>();
  let set2 = vec2.clone().into_iter().collect::<BitSet>();
  let hset1 = vec1.into_iter().collect::<HashSet<_>>();
  let set3 = vec2.into_iter()
    .filter(|x| hset1.contains(x))
    .collect::<HashSet<_>>()
    .into_iter()
    .collect::<BitSet>();

  &set1 & &set2 == set3 && set1 & set2 == set3
}

#[test]
fn bitand_large() {
  let set1 = (0..1000000).step_by(5).collect::<BitSet>();
  let set2 = (0..1000000).step_by(3).collect::<BitSet>();
  let set3 = (0..1000000).step_by(15).collect::<BitSet>();

  assert_eq!(set1.clone() & set2.clone(), set3);

  assert_eq!(set2 & set1, set3);
}

#[quickcheck]
fn set_difference_prop(vec1: Vec<u16>, vec2: Vec<u16>) -> bool {
  let vec1 = vec1.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let vec2 = vec2.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let set1 = vec1.clone().into_iter().collect::<BitSet>();
  let set2 = vec2.clone().into_iter().collect::<BitSet>();
  let hset2 = vec2.into_iter().collect::<HashSet<_>>();
  let set3 = vec1.into_iter().filter(|x| !hset2.contains(x)).collect::<BitSet>();

  &set1 - &set2 == set3 && set1 - set2 == set3
}

#[test]
fn set_difference_large() {
  let set1 = (0..1000000).step_by(5).collect::<BitSet>();
  let set2 = (0..1000000).step_by(3).collect::<BitSet>();

  let set3 = (0..1000000).step_by(5)
    .filter(|x| x % 3 != 0)
    .collect::<BitSet>();

  let set4 = (0..1000000).step_by(3)
    .filter(|x| x % 5 != 0)
    .collect::<BitSet>();

  assert_eq!(set1.clone() - set2.clone(), set3);

  assert_eq!(set2 - set1, set4);
}

#[quickcheck]
fn bitxor_prop(vec1: Vec<u16>, vec2: Vec<u16>) -> bool {
  let vec1 = vec1.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let vec2 = vec2.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let set1 = vec1.clone().into_iter().collect::<BitSet>();
  let set2 = vec2.clone().into_iter().collect::<BitSet>();
  let hset2 = vec2.clone().into_iter().collect::<HashSet<_>>();
  let mut set3 = vec1.clone().into_iter().chain(vec2).collect::<HashSet<_>>();
  for x in vec1 {
    if hset2.contains(&x) {
      set3.remove(&x);
    }
  }
  let set3 = set3.into_iter().collect::<BitSet>();

  &set1 ^ &set2 == set3 && set1 ^ set2 == set3
}

#[test]
fn bitxor_large() {
  let set1 = (0..1000000).step_by(5).collect::<BitSet>();
  let set2 = (0..1000000).step_by(3).collect::<BitSet>();

  let set3 = (0..1000000).step_by(3)
    .chain((0..1000000).step_by(5))
    .filter(|x| x % 15 != 0)
    .collect::<BitSet>();

  assert_eq!(set1 ^ set2, set3);
}

#[quickcheck]
fn is_subset_refl_prop(vec: Vec<u16>) -> bool {
  let vec = vec.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let set = vec.into_iter().collect::<BitSet>();
  set.is_subset(&set)
}

#[quickcheck]
fn is_subset_one_more_prop(om: helper::OneMore) -> bool {
  let vec1 = om.vec.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let mut vec2 = vec1.clone();
  vec2.push(om.x as usize);
  let set1 = vec1.into_iter().collect::<BitSet>();
  let set2 = vec2.into_iter().collect::<BitSet>();

  set1.is_subset(&set2) && !set2.is_subset(&set1)
}

#[quickcheck]
fn is_subset_vec_prop(tv: helper::TwoVec) -> bool {
  let vec1 = tv.vec1.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let mut vec2 = tv.vec2.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  vec2.extend(vec1.clone());
  let set1 = vec1.into_iter().collect::<BitSet>();
  let set2 = vec2.into_iter().collect::<BitSet>();

  set1.is_subset(&set2) && (set1.len() == set2.len() || !set2.is_subset(&set1))
}

#[quickcheck]
fn is_propert_subset_not_refl_prop(vec: Vec<u16>) -> bool {
  let vec = vec.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let set = vec.into_iter().collect::<BitSet>();
  !set.is_proper_subset(&set)
}

#[quickcheck]
fn is_proper_subset_one_more_prop(om: helper::OneMore) -> bool {
  let vec1 = om.vec.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let mut vec2 = vec1.clone();
  vec2.push(om.x as usize);
  let set1 = vec1.into_iter().collect::<BitSet>();
  let set2 = vec2.into_iter().collect::<BitSet>();

  set1.is_proper_subset(&set2) && !set2.is_proper_subset(&set1)
}

#[quickcheck]
fn is_proper_subset_vec_prop(tv: helper::TwoVec) -> bool {
  let vec1 = tv.vec1.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  let mut vec2 = tv.vec2.into_iter().map(|x| x as usize).collect::<Vec<_>>();
  vec2.extend(vec1.clone());
  let set1 = vec1.into_iter().collect::<BitSet>();
  let set2 = vec2.into_iter().collect::<BitSet>();

  set1.is_proper_subset(&set2) &&
    (set1.len() == set2.len() || !set2.is_proper_subset(&set1))
}