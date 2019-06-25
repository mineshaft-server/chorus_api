use crate::chunk_data::chunk_block::ChunkBlock;
use crate::chunk_data::constants::*;

#[test]
pub fn reading_block_info_works() {
  let mut chunk_block = ChunkBlock::new();

  // Setup for even index test
  chunk_block.ids[0] = 1;
  chunk_block.extended[0] = 3;
  chunk_block.data[0] = 2;

  let mut block_info = chunk_block.get(0,0,0);

  assert_eq!(block_info.0, 0x0301);
  assert_eq!(block_info.1, 2);

  // Setup for odd index test
  chunk_block.ids[0] = 0;
  chunk_block.extended[0] = 0;
  chunk_block.data[0] = 0;

  chunk_block.ids[1] = 1;
  chunk_block.extended[0] = 0x30;
  chunk_block.data[0] = 0x20;

  // Get the info
  block_info = chunk_block.get(1,0,0);

  assert_eq!(block_info.0, 0x0301);
  assert_eq!(block_info.1, 2);
}

#[test]
pub fn writing_block_info_works() {
  let mut chunk_block = ChunkBlock::new();

  chunk_block.set(0,0,0, (1, 3));
  assert_eq!(chunk_block.ids[0], 1);
  assert_eq!(chunk_block.data[0], 3);

  chunk_block.ids[0] = 0;
  chunk_block.data[0] = 0;


  chunk_block.set(1,0,0, (1, 3));
  assert_eq!(chunk_block.ids[1], 1);
  assert_eq!(chunk_block.data[0], 0x30);

}

#[test]
pub fn writing_block_info_on_valid_block_return_true() {
  let mut chunk_block = ChunkBlock::new();
  assert_eq!(chunk_block.set(1,34,2, (0, 0)), true);
}

#[test]
pub fn writing_block_info_on_invalid_block_returns_false() {
  let mut chunk_block = ChunkBlock::new();
  assert_eq!(chunk_block.set(1,264,2, (0, 0)), false);
  assert_eq!(chunk_block.set(1,-264,2, (0, 0)), false);
}

#[test]
pub fn writing_block_id_works() {
  let mut chunk_block = ChunkBlock::new();

  chunk_block.set_block(0,0,0, 1);
  assert_eq!(chunk_block.ids[0], 1);
  assert_eq!(chunk_block.data[0], 0);

  chunk_block.ids[0] = 0;


  chunk_block.set_block(1,0,0, 1);
  assert_eq!(chunk_block.ids[1], 1);
}

#[test]
pub fn writing_block_id_on_valid_block_returns_true() {
  let mut chunk_block = ChunkBlock::new();
  assert_eq!(chunk_block.set_block(5,5,5, 0), true);
}

#[test]
pub fn writing_block_id_on_invalid_block_returns_false() {
  let mut chunk_block = ChunkBlock::new();
  assert_eq!(chunk_block.set_block(5,765,5, 0), false);
  assert_eq!(chunk_block.set_block(5,-765,5, 0), false);
}

#[test]
pub fn writing_metadata_works() {
  let mut chunk_block = ChunkBlock::new();
  chunk_block.set_metadata(0,0,0, 1);
  assert_eq!(chunk_block.data[0], 1);
  chunk_block.data[0] = 0;
  chunk_block.set_metadata(1,0,0, 1);
  assert_eq!(chunk_block.data[0], 0x10);
}

#[test]
pub fn writing_metadata_on_valid_block_returns_true() {
  let mut chunk_block = ChunkBlock::new();
  assert_eq!(chunk_block.set_metadata(5,5,5, 0), true);
}

#[test]
pub fn writing_metadata_on_invalid_block_returns_false() {
  let mut chunk_block = ChunkBlock::new();
  assert_eq!(chunk_block.set_metadata(5,765,5, 0), false);
  assert_eq!(chunk_block.set_metadata(5,-765,5, 0), false);
}