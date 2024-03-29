use std::vec::Vec;

use super::constants::*;
use super::util::{
  BlockTuple,
  get_index
};

const EXTRA_SIZE: i8 = 4;

#[cfg(test)]
pub struct ChunkBlock {
  pub ids: Vec<u8>,
  pub extended: Vec<u8>,
  pub data: Vec<u8>
}

#[cfg(not(test))]
pub struct ChunkBlock {
  ids: Vec<u8>,
  extended: Vec<u8>,
  data: Vec<u8>
}

impl ChunkBlock {
  pub fn new() -> ChunkBlock {
    return ChunkBlock {
      ids: vec![0; BLOCK_COUNT as usize],
      extended: vec![0; (BLOCK_COUNT / 2) as usize],
      data: vec![0; (BLOCK_COUNT / 2) as usize],
    }
  }

  pub fn from(ids: Vec<u8>, extended: Vec<u8>, data: Vec<u8>) -> ChunkBlock {
    return ChunkBlock {
      ids: ids,
      extended: extended,
      data: data,
    }
  }

  pub fn get(&self, x: i32, y: i32, z: i32) -> Option<BlockTuple> {
    let mut blockdata: u16 = 0;
    let mut data: u8 = 0;

    // If x and z are within bounds
    if x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH {
      // If y is within bounds
      if y < CHUNK_HEIGHT && y > -1 {
        let index = get_index(x, y, z);
        let condition = index % 2 == 0;

        // Build the block id
        let mut extended = self.extended[(index / 2)] as u16;
        if condition {
          extended &= 0xF;
        } else {
          extended = (extended & 0xF0) >> 4;
        }
        blockdata = self.ids[index] as u16;
        blockdata = blockdata | (extended << 8);

        // Grab the metadata for the block as well
        data = self.data[(index / 2)];
        if condition {
          data = data & 0x0F;
        } else {
          data = (data & 0xF0) >> EXTRA_SIZE;
        }
      }

      return Some(BlockTuple{block: blockdata, metadata: data});
    }

    // X or Z is out of bounds. Return none
    return None;
  }

  pub fn set(&mut self, x: i32, y: i32, z: i32, block: (u16, u8)) -> bool {
    // If x and z are within bounds
    // If the index is valid, set the block data.
    // Otherwise ignore
    if  x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH && y < CHUNK_HEIGHT && y > -1 {
      let index = get_index(x, y, z);
      let nibble_index = index / 2;
      let condition = index % 2 == 0;

      // Store the block with the extended block id
      let extended = ((block.0 & 0x0F00) >> 8) as u8;
      if condition {
        self.extended[nibble_index] = extended | (self.extended[nibble_index] & 0xF0);
      } else {
        self.extended[nibble_index] = (extended << 4) | (self.extended[nibble_index] & 0x0F);
      }
      self.ids[index] = (block.0 & 0x00FF) as u8;

      // Set the metadata
      let data = if condition { block.1 } else { block.1 << 4 };
      self.data[nibble_index] = data | (self.data[nibble_index] & (if condition { 0xF0 } else {0x0F}));
      return true;
    }

    return false;
  }

  pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: u16) -> bool {
    // If x and z are within bounds
    // If the index is valid, set the block data.
    // Otherwise ignore
    if x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH && y < CHUNK_HEIGHT && y > -1 {
      let index = get_index(x, y, z);
      let nibble_index = index / 2;
      let condition = index % 2 == 0;

      // Store the block with the extended block id
      let extended = ((block & 0x0F00) >> 8) as u8;
      if condition {
        self.extended[nibble_index] = extended | (self.extended[nibble_index] & 0xF0);
      } else {
        self.extended[nibble_index] = (extended << 4) | (self.extended[nibble_index] & 0x0F);
      }
      self.ids[index] = (block & 0x00FF) as u8;

      // Clear the metadata
      self.data[nibble_index] = self.data[nibble_index] & (if condition { 0xF0 } else {0x0F});
      return true;
    }

    return false;
  }

  pub fn set_metadata(&mut self, x: i32, y: i32, z: i32, data: u8) -> bool {
    // If the index is valid, set the metadata.
    if x > -1 && x < CHUNK_WIDTH && z > -1 && z < CHUNK_WIDTH && y < CHUNK_HEIGHT && y > -1 {
      let index = get_index(x, y, z);
      let nibble_index = index / 2;
      let condition = index % 2 == 0;

      // Set the metadata
      let data = if condition { data } else { data << 4 };
      self.data[nibble_index] = data | (self.data[nibble_index] & (if condition { 0xF0 } else {0x0F}));
      return true;
    }
    return false
  }
}