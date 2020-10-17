use quickcheck::{Arbitrary, Gen};

#[derive(Debug, Clone)]
pub struct OneMore {
  pub vec: Vec<u16>,
  pub x: u16,
}

impl Arbitrary for OneMore {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let vec = Vec::<u16>::arbitrary(g);
    let x = loop {
      let x = u16::arbitrary(g);
      if vec.contains(&x) {
        continue;
      }
      break x;
    };

    Self { vec, x }
  }
}

#[derive(Debug, Clone)]
pub struct TwoVec {
  pub vec1: Vec<u16>,
  pub vec2: Vec<u16>,
}

impl Arbitrary for TwoVec {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let vec1 = Vec::<u16>::arbitrary(g);
    let mut vec2 = Vec::<u16>::arbitrary(g);
    if vec2.is_empty() {
      vec2.push(u16::arbitrary(g));
    }

    Self { vec1, vec2 }
  }
}