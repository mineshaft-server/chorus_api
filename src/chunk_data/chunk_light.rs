use std::vec::Vec;
use super::constants::{
  CHUNK_WIDTH,
  BLOCK_COUNT,
};

#[cfg(test)]
pub struct ChunkLight {
  pub block: Vec<u8>,
  pub sky: Vec<u8>
}

#[cfg(not(test))]
pub struct ChunkLight {
  block: Vec<u8>,
  sky: Vec<u8>
}

impl ChunkLight {
  pub fn new() -> ChunkLight {
    return ChunkLight {
      block: vec![0; (BLOCK_COUNT / 2) as usize],
      sky: vec![0; (BLOCK_COUNT / 2) as usize]
    }
  }

  pub fn from(block: Vec<u8>, sky: Vec<u8>) -> ChunkLight {
    return ChunkLight {
      block: block,
      sky: sky
    }
  }

  pub fn get_sky(&self, x: i32, y: i32, z: i32) -> u8 {
    let index = ChunkLight::get_index(x, y, z);
    if index < BLOCK_COUNT {
      if index > -1 {
        if index % 2 == 0 {
          return self.sky[(index / 2) as usize] & 0x0F;
        } else {
          return (self.sky[(index / 2) as usize] & 0xF0) >> 4;
        }
      } else {
        return 0;
      }
    } else {
      return 15;
    }
  }

  pub fn set_sky(&mut self, x: i32, y: i32, z: i32, light: u8) -> bool{
    let index = ChunkLight::get_index(x, y, z);
    if index < BLOCK_COUNT && index > -1 {
      let condition = index % 2 == 0;
      let actual_light = if condition { light & 0x0F } else { (light & 0x0F) << 4};
      let current = self.sky[(index / 2) as usize];
      self.sky[(index / 2) as usize] = actual_light | (current & (if condition { 0xF0 } else {0x0F}));
      return true;
    }
    return false;
  }

  pub fn set_block(&mut self, x: i32, y: i32, z: i32, light: u8) -> bool{
    let index = ChunkLight::get_index(x, y, z);
    if index < BLOCK_COUNT && index > -1 {
      let condition = index % 2 == 0;
      let actual_light = if condition { light & 0x0F } else { (light & 0x0F) << 4};
      let current = self.block[(index / 2) as usize];
      self.block[(index / 2) as usize] = actual_light | (current & (if condition { 0xF0 } else {0x0F}));
      return true;
    }
    return false;
  }

  pub fn get_block(&self, x: i32, y: i32, z: i32) -> u8 {
    let index = ChunkLight::get_index(x, y, z);
    println!("INDEX {}", index);
    if index < BLOCK_COUNT && index > -1 {
      if index % 2 == 0 {
          return self.block[(index / 2) as usize] & 0x0F;
        } else {
          return (self.block[(index / 2) as usize] & 0xF0) >> 4;
        }
    } else {
      return 0;
    }
  }

  fn get_index(x: i32, y: i32, z: i32) -> i32 {
    return (y * CHUNK_WIDTH * CHUNK_WIDTH) + (z * CHUNK_WIDTH) + x ;
  }
}