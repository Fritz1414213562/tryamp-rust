pub const DEFAULT_BUFFER_SCALE   : usize = 10; // read 10 frames at once
pub const COORDINATE_BLOCK_OFFSET: usize = 8;  // size of block-size description (i32 x 2)
pub const COORDINATE_BLOCK_SCALE : usize = 3;  // 3 dimensions, (x, y, z)

pub const DCD_SECOND_BLOCK: &[u8] = b"======================= Written by DCD Writer in try-amp =======================";
pub const DCD_SECOND_BLOCKSIZE: i32 = (DCD_SECOND_BLOCK.len() + 4) as i32;
