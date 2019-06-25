use rand::prelude::*;
use std::collections::hash_set::Iter;
use std::collections::HashSet;

use crate::util::identifier::Identifier;

pub struct Tags {
  set: HashSet<Identifier>,
}

impl Tags {
  pub fn new() -> Tags {
    return Tags {
      set: HashSet::new(),
    }
  }

  pub fn clear(&mut self) {
    self.set.clear()
  }

  pub fn contains(&self, id: &Identifier) -> bool {
    return self.set.contains(id);
  }

  pub fn empty(&self) -> bool {
    return self.set.is_empty();
  }

  pub fn insert(&mut self, id: Identifier) -> bool {
    return self.set.insert(id);
  }

  pub fn iter(&self) -> Iter<Identifier> {
    return self.set.iter();
  }

  pub fn len(&self) -> usize {
    return self.set.len();
  }

  pub fn nth(&self, idx: usize) -> Option<&Identifier> {
    return self.iter().nth(idx);
  }

  pub fn random(&self, rand: &mut StdRng) -> Option<&Identifier> {
    let count = self.set.len();
    let idx = (rand.next_u64() as usize) % count;
    return self.iter().nth(idx);
  }

  pub fn remove(&mut self, id: &Identifier) -> bool {
    return self.set.remove(id);
  }
}