//! tiled_json is here to help you load Tiled maps!
//! 
//! The primary idea of this library is getting the data loaded into a more
//! manageable format.  This data is not optimized toward any specific use case
//! and as such should be used only as an intermediate format before 
//! transferring into more useful structures.  This library exists soley to 
//! facilitate the loading of Tiled maps and is designed to be read-only.  The 
//! data structures within have no extended functionality or any inherent purpose
//! other than reading data stored in Tiled JSON maps. 
//! 
//! **This library supports loading compressed and base64 encoded maps.**
//! 
//! **This library does NOT support loading wangsets, chunks, terrains, 
//! infinite maps, or external object templates.**  This means when you export
//! a map to JSON, you must be sure to 
//! - Embed Tilesets, and
//! - Detach Templates, and   
//! - Resolve Object Types and Properties (optional).   
//! 
//! Every field of every struct is public.  In order to get data, you
//! may access the fields directly or use the methods by the same name. 
//! Each variable is named according to its Tiled JSON representation.  
//! Numerous convenient functions are available for accessing data that may be
//! otherwise difficult or verbose to access.
//! 
//! All enums can be converted into string slices if you need them via the 
//! to_string() method from implementing Display.
//! 
//! 
//! This is what the data tree looks like:
//! 
//!         Map
//!             Layers  
//!                 Tile Layers
//!                     Data (gids corresponding to some tileset)
//!                 Object Groups
//!                     Objects
//!                 Image Layers (images directly on map)
//!                 Groups (groups of layers)
//!             Tilesets
//!                 Tiles
//!                 Animations
//!                 Collisions
//! 
//! ```tiled_json::load_map(file: &str)``` is the one and only entry point into 
//! this library.
//! 
//! Typically, we want to load the map, we'll capture it to a variable.  Then we
//! might loop through all of the tilesets and translate them to our own structures
//! and then do the same for our layers.  Here is what some code may look like:
//! ```
//! let map = tiled_json::load_map("map1.json").unwrap();
//! let height = map.height();
//! let width = map.width();
//! // CREATE INTERNAL MAP STRUCTURE HERE, THEN
//! for ls in map.layers().iter() {
//!     let layer_darkness = ls.get_property("darkness").get_float();
//!     let layer_weather  = ls.get_property("weather").get_string();
//!     if ls.is_tile_layer() {
//!         let data = ls.get_data().unwrap();
//!         for n in data.iter() {
//!             let flip_bits = tiled_json::gid_flipped_hvd( *n );
//!             let gid = tiled_json::gid_without_flags( *n );
//!             let ts = map.tileset_by_gid( gid ).unwrap();
//!             let coords = ts.coord_by_gid(gid);
//!             if let Option::Some(tile) = ts.tile_by_gid( gid ) {
//!                 // these are specific overrides of the tileset for a specific tile
//!                 // these can be accessed from the tileset for easy access later.
//!                 let anim_coords = tile.get_anim( MSECS_SINCE_WORLD_CREATION ).unwrap(); // this may not exist 
//!                 let collision = tile.objectgroup().unwrap(); // this may not exist
//!                 let properties = tile.get_property_vector();
//!                 // do stuff!
//!             }
//!             
//!             // check if an existing instance exists of this and if not, create
//!             // a new instance for reference later.
//!             // Add value to our map.
//!         }
//!    
//!     } else if ls.is_object_group() {
//!         let objs = ls.get_objects_vector().unwrap();
//!         let dro  = ls.get_draworder().unwrap(); 
//!         // See object properties to know what is relevant to you.
//! 
//!     } else if ls.is_image_layer() {
//!         let img = ls.get_image().unwrap();
//!         let tsc = ls.get_transparentcolor().unwrap(); // this can actually be None so use with caution.
//!         // Determine what to do with the image.
//! 
//!     } else if ls.is_group() {
//!         let grp = ls.get_group().unwrap();
//!         // If you have groups in your map, then you know what to do with them.
//!     }
//! }
//! for ts in map.tilesets().iter() {
//!     /*  load stuff here
//!         store textures
//!         organize your internal data
//!     */  
//! }
//! 
//! ```
//! 

#![allow(dead_code)]

pub mod color;
pub mod layer;
mod layerreader;
pub mod map;
pub mod object;
pub mod property;
pub mod tileset;

pub use crate::color::*;
pub use crate::layer::*;
pub use crate::map::*;
pub use crate::object::*;
pub use crate::property::*;
pub use crate::tileset::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub const HORZ_FLIP_FLAG: u32 = 0x8000_0000;
pub const VERT_FLIP_FLAG: u32 = 0x4000_0000;
pub const DIAG_FLIP_FLAG: u32 = 0x2000_0000;

/// It is all exposed through this function--load_map() which takes a filename 
/// as a string slice and (hopefully) gives you a tiled_json::Map object in 
/// return.  
/// ```
/// let map = tiled_json::load_map("map1.json");
/// if map.is_err() {
///     /* do some error handling */
///     /* for std::io::Error */
/// }
/// ```
pub fn load_map(file: &str) -> Result<Map, std::io::Error> {
    let file = File::open(file)?;

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let map: Map = serde_json::from_str(&contents)?;
    Ok(map)
}

/// The gid in tile layer data tells us if the tile at a location is flipped
/// on the horizontal axis.  This function takes that information and determines 
/// that for you by returning a boolean.
#[inline]
pub fn gid_flipped_horizontally(gid: u32) -> bool {
    gid & HORZ_FLIP_FLAG > 0
}

/// The gid in tile layer data tells us if the tile at a location is flipped
/// on the vertical axis.  This function takes that information and determines 
/// that for you by returning a boolean.
#[inline]
pub fn gid_flipped_vertically(gid: u32) -> bool {
    gid & VERT_FLIP_FLAG > 0
}

/// The gid in tile layer data tells us if the tile at a location is flipped
/// over the diagonal x=y axis.  This function takes that information and determines 
/// that for you by returning a boolean.
#[inline]
pub fn gid_flipped_diagonally(gid: u32) -> bool {
    gid & DIAG_FLIP_FLAG > 0
}


/// The gid in tile layer data tells us if the tile at a location is flipped
/// on the following axes: horizontal, vertical, and diagonal.  
/// This function takes that information and determines that for you by returning a
/// set of three booleans: (horizontal flip, vertical flip, diagonal flip).
#[inline]
pub fn gid_flipped_hvd(gid: u32) -> (bool,bool,bool) {
    (gid & HORZ_FLIP_FLAG > 0,
     gid & VERT_FLIP_FLAG > 0,
     gid & DIAG_FLIP_FLAG > 0)
}

/// Submit a gid and recieve a copy with the flags removed.
#[inline]
pub fn gid_without_flags(gid: u32) -> u32 {
    gid & !(HORZ_FLIP_FLAG | VERT_FLIP_FLAG | DIAG_FLIP_FLAG)
}