use super::constants::CHUNK_WIDTH;

#[derive(Eq,PartialEq,Debug)]
pub struct LightTuple {
  pub block: u8,
  pub sky: u8,
}

#[derive(Eq,PartialEq,Debug)]
pub struct BlockTuple {
  pub block: u16,
  pub metadata: u8
}

pub fn get_index(x: i32, y: i32, z: i32) -> usize {
  return ((y * CHUNK_WIDTH * CHUNK_WIDTH) + (z * CHUNK_WIDTH) + x) as usize;
}