use criterion::{black_box, criterion_group, criterion_main, Criterion, Fun};
use bittyset::BitSet as BittySet;
use bit_set::BitSet;
use rand::Rng;

fn random_vec(len: usize, nbits: usize) -> Vec<usize> {
  let mut rng = rand::thread_rng();
  (0..len).map(|_| rng.gen_range(0usize, 1 << nbits)).collect()
}

fn insert_benchmark(c: &mut Criterion) {
  c.bench_functions(
    "insert",
    vec![
      Fun::new("bit_set", |b, _| {
        let mut set = BitSet::new();
        b.iter_with_large_setup(|| random_vec(10000, 12), |input| {
          set.clear();
          
          for i in input {
            set.insert(i);
          }
        });
      }),

      Fun::new("bittyset", |b, _| {
        let mut set = BittySet::<u32>::new();
        b.iter_with_large_setup(|| random_vec(10000, 12), |input| {
          set.clear();
          
          for i in input {
            set.insert(i);
          }
        });
      })
    ],
    ());
}

fn remove_benchmark(c: &mut Criterion) {
  c.bench_functions(
    "remove",
    vec![
      Fun::new("bit_set", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 12).into_iter().collect::<BitSet>(),
               random_vec(5000, 12)
             ),
          |(mut set, remove)| {
            for i in remove {
              set.remove(i);
            }
          }
        );
      }),

      Fun::new("bittyset", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 12).into_iter().collect::<BittySet<u32>>(),
               random_vec(5000, 12)
             ),
          |(mut set, remove)| {
            for i in remove {
              set.remove(i);
            }
          }
        );
      })
    ],
    ());
}

fn contains_benchmark(c: &mut Criterion) {
  let insert = random_vec(10000, 12);
  let contains = random_vec(5000, 12);

  c.bench_functions(
    "contains",
    vec![
      Fun::new("bit_set", |b, (insert, contains): &(Vec<usize>, _)| {
        let set = insert.iter().cloned().collect::<BitSet>();
        b.iter(|| {
          for &i in contains {
            black_box(set.contains(i));
          }
        });
      }),

      Fun::new("bittyset", |b, (insert, contains): &(Vec<usize>, _)| {
        let set = insert.iter().cloned().collect::<BittySet<u32>>();
        b.iter(|| {
          for &i in contains {
            black_box(set.contains(i));
          }
        });
      })
    ],
    (insert, contains));
}

fn union_benchmark(c: &mut Criterion) {
  c.bench_functions(
    "union",
    vec![
      Fun::new("bit_set", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 16).into_iter().collect::<BitSet>(),
               random_vec(10000, 16).into_iter().collect::<BitSet>()
             ),
          |(mut set1, set2)| {
            set1.union_with(&set2);
          }
        );
      }),

      Fun::new("bittyset", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 16).into_iter().collect::<BittySet<u32>>(),
               random_vec(10000, 16).into_iter().collect::<BittySet<u32>>()
             ),
          |(mut set1, set2)| {
            set1.union_with(&set2);
          }
        );
      })
    ],
    ());
}

fn intersection_benchmark(c: &mut Criterion) {
  c.bench_functions(
    "intersection",
    vec![
      Fun::new("bit_set", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 16).into_iter().collect::<BitSet>(),
               random_vec(10000, 16).into_iter().collect::<BitSet>()
             ),
          |(mut set1, set2)| {
            set1.intersect_with(&set2);
          }
        );
      }),

      Fun::new("bittyset", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 16).into_iter().collect::<BittySet<u32>>(),
               random_vec(10000, 16).into_iter().collect::<BittySet<u32>>()
             ),
          |(mut set1, set2)| {
            set1.intersect_with(&set2);
          }
        );
      })
    ],
    ());
}

fn difference_benchmark(c: &mut Criterion) {
  c.bench_functions(
    "difference",
    vec![
      Fun::new("bit_set", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 16).into_iter().collect::<BitSet>(),
               random_vec(10000, 16).into_iter().collect::<BitSet>()
             ),
          |(mut set1, set2)| {
            set1.difference_with(&set2);
          }
        );
      }),

      Fun::new("bittyset", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 16).into_iter().collect::<BittySet<u32>>(),
               random_vec(10000, 16).into_iter().collect::<BittySet<u32>>()
             ),
          |(mut set1, set2)| {
            set1.difference_with(&set2);
          }
        );
      })
    ],
    ());
}

fn symmetric_difference_benchmark(c: &mut Criterion) {
  c.bench_functions(
    "symmetric_difference",
    vec![
      Fun::new("bit_set", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 16).into_iter().collect::<BitSet>(),
               random_vec(10000, 16).into_iter().collect::<BitSet>()
             ),
          |(mut set1, set2)| {
            set1.symmetric_difference_with(&set2);
          }
        );
      }),

      Fun::new("bittyset", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(10000, 16).into_iter().collect::<BittySet<u32>>(),
               random_vec(10000, 16).into_iter().collect::<BittySet<u32>>()
             ),
          |(mut set1, set2)| {
            set1.symmetric_difference_with(&set2);
          }
        );
      })
    ],
    ());
}

/// Benchmark reflexivity of subset relation.
fn is_subset_refl_benchmark(c: &mut Criterion) {
  c.bench_functions(
    "is_subset(self)",
    vec![
      Fun::new("bit_set", |b, _| {
        b.iter_with_large_setup(
          || random_vec(10000, 16).into_iter().collect::<BitSet>(),
          |set| {
            black_box(set.is_subset(&set));
          }
        );
      }),

      Fun::new("bittyset", |b, _| {
        b.iter_with_large_setup(
          || random_vec(10000, 16).into_iter().collect::<BittySet<u32>>(),
          |set| {
            black_box(set.is_subset(&set));
          }
        );
      })
    ],
    ());
}

fn is_subset_benchmark(c: &mut Criterion) {
  c.bench_functions(
    "is_subset",
    vec![
      Fun::new("bit_set", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(100, 12).into_iter().collect::<BitSet>(),
               random_vec(10000, 12).into_iter().collect::<BitSet>()
             ),
          |(set1, set2)| {
            black_box(set1.is_subset(&set2));
          }
        );
      }),

      Fun::new("bittyset", |b, _| {
        b.iter_with_large_setup(
          || ( random_vec(100, 12).into_iter().collect::<BittySet<u32>>(),
               random_vec(10000, 12).into_iter().collect::<BittySet<u32>>()
             ),
          |(set1, set2)| {
            black_box(set1.is_subset(&set2));
          }
        );
      })
    ],
    ());
}

criterion_group!(benches,
  insert_benchmark,
  remove_benchmark,
  contains_benchmark,
  union_benchmark,
  intersection_benchmark,
  difference_benchmark,
  symmetric_difference_benchmark,
  is_subset_benchmark,
  is_subset_refl_benchmark);
criterion_main!(benches);