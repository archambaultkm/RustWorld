extern crate serde;
extern crate serde_json;

use std::error::Error;
use serde::{Deserialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct BlockFaceConfig {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

#[derive(Debug, Deserialize)]
pub struct BlockConfig {
    pub top: BlockFaceConfig,
    pub bottom: BlockFaceConfig,
    pub side: BlockFaceConfig,
}

#[derive(Debug, Deserialize)]
pub struct BlockTypeConfig {
    pub blocks: std::collections::HashMap<String, BlockConfig>,
}

pub fn load_block_config() -> Result<BlockTypeConfig, Box<dyn Error>> {
    let file = File::open("resources/data/spritesheet.json")?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| Box::new(e) as Box<dyn Error>)
}

impl BlockTypeConfig {
    pub fn get_texture_coordinates(&self, block_type: &str, face: &str) -> (f32, f32, f32, f32) {
        if let Some(block_config) = self.blocks.get(block_type) {
            let face_config = match face {
                "top" => &block_config.top,
                "bottom" => &block_config.bottom,
                "side" => &block_config.side,
                _ => &block_config.top, // Default to "top" if an invalid face is specified
            };
            (face_config.x as f32, face_config.y as f32, face_config.w as f32, face_config.h as f32)
        } else {
            (1.0, 1.0, 160.0, 160.0) // Default to a generic texture coordinate // TODO hard-coded width and length
        }
    }
}


