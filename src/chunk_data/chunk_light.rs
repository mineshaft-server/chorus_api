use std::vec::Vec;
use super::constants::{
  CHUNK_WIDTH,
  CHUNK_HEIGHT,
  BLOCK_COUNT,
};

use super::util::{
  LightTuple,
  get_index
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

  pub fn get(&self, x: i32, y: i32, z: i32) -> Option<LightTuple> {
    if x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH {
      if y < CHUNK_HEIGHT {
        if y > -1 {
          let index = get_index(x, y, z);
          let nibble_index = index / 2;
          let condition = index % 2 == 0;
          let block = if condition { self.block[nibble_index] & 0x0F } else { (self.block[nibble_index] & 0xF0) >> 4 };
          let sky = if condition { self.sky[nibble_index] & 0x0F } else { (self.sky[nibble_index] & 0xF0) >> 4 };
          return Some(LightTuple{block: block, sky: sky});
        }
        return Some(LightTuple{block: 0, sky: 0});
      }
      return Some(LightTuple{block: 0, sky: 15});
    }

    return None;
  }

  pub fn get_sky(&self, x: i32, y: i32, z: i32) -> Option<u8> {
    if x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH {
      if y < CHUNK_HEIGHT {
        if y > -1 {
          let index = get_index(x, y, z);
          if index % 2 == 0 {
            return Some(self.sky[(index / 2)] & 0x0F);
          } else {
            return Some((self.sky[(index / 2)] & 0xF0) >> 4);
          }
        } else {
          return Some(0);
        }
      } else {
        return Some(15);
      }
    }

    return None;
  }

  pub fn set_sky(&mut self, x: i32, y: i32, z: i32, light: u8) -> bool {
    if x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH && y < CHUNK_HEIGHT && y > -1 {
      let index = get_index(x, y, z);
      let condition = index % 2 == 0;
      let actual_light = if condition { light & 0x0F } else { (light & 0x0F) << 4};
      let current = self.sky[(index / 2)];
      self.sky[(index / 2)] = actual_light | (current & (if condition { 0xF0 } else {0x0F}));
      return true;
    }
    return false;
  }

  pub fn set_block(&mut self, x: i32, y: i32, z: i32, light: u8) -> bool{
    if x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH && y < CHUNK_HEIGHT && y > -1 {
      let index = get_index(x, y, z);
      let condition = index % 2 == 0;
      let actual_light = if condition { light & 0x0F } else { (light & 0x0F) << 4};
      let current = self.block[(index / 2)];
      self.block[(index / 2)] = actual_light | (current & (if condition { 0xF0 } else {0x0F}));
      return true;
    }
    return false;
  }

  pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<u8> {
    if x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH {
      if y < CHUNK_HEIGHT && y > -1 {
        let index = get_index(x, y, z);
        if index % 2 == 0 {
            return Some(self.block[(index / 2)] & 0x0F);
          } else {
            return Some((self.block[(index / 2)] & 0xF0) >> 4);
          }
      } else {
        return Some(0);
      }
    }

    return None;
  }
}