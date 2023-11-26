use crate::core::lib::PolygonMode;
use crate::core::lib::PolygonMode::*;

pub const TITLE : &str = "RustWorld";

// settings
pub const WINDOW_WIDTH : u32 = 800;
pub const WINDOW_HEIGHT : u32 = 600;
pub const CHUNK_SIZE : usize = 16;
pub const NUM_CHUNKS : usize = 6;
pub const MAX_CHUNK_HEIGHT : usize = 10;
pub const POLYGON_MODE : PolygonMode = Fill;