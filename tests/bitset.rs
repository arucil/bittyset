use bittyset::BitSet;
use pretty_assertions::{assert_eq, assert_ne};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

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
  let set = vec![37,0,14,7,14].into_iter().collect::<BitSet>();

  assert_eq!(set.iter().collect::<Vec<_>>(), vec![0,7,14,37]);
}

#[test]
fn eq() {
  let set1 = vec![7,1,4,5,41,4].into_iter().collect::<BitSet>();
  let mut set2 = vec![7,1,41,4].into_iter().collect::<BitSet>();

  assert_ne!(set1, set2);

  set2.insert(5);

  assert_eq!(set1, set2);

  set2.remove(41);

  assert_ne!(set1, set2);

  assert_eq!(<BitSet>::new(), <BitSet>::new());

  let set1 = vec![63].into_iter().collect::<BitSet>();
  let set2 = vec![63].into_iter().collect::<BitSet>();

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
  let set1 = vec![7,1,4,5,41,4].into_iter().collect::<BitSet>();
  let mut set2 = vec![7,1,41,4].into_iter().collect::<BitSet>();

  assert_ne!(my_hash(&set1), my_hash(&set2));

  set2.insert(5);

  assert_eq!(my_hash(&set1), my_hash(&set2));

  set2.remove(41);

  assert_ne!(my_hash(&set1), my_hash(&set2));

  assert_eq!(<BitSet>::new(), <BitSet>::new());

  let set1 = vec![63].into_iter().collect::<BitSet>();
  let set2 = vec![63].into_iter().collect::<BitSet>();

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