//! The Map struct is the primary interface to the rest of the library's data.
//! 
//! Through it, one can access tilesets and layers (among the rest of map's 
//! data).  All variable names are the same as those listed in the documentation of 
//! Tiled JSON format:
//! > <https://doc.mapeditor.org/en/stable/reference/json-map-format/#map>   
//! 
//! The most valuable functions beside the standard getters will be:
//! 
//!         tiled_json::Map::layers(&self) -> &Vec<tiled_json::Layer>;
//!         tiled_json::Map::layer_by_name(&self, &str) -> Option<&Layer>;
//! 
//!         tiled_json::Map::tilesets(&self) -> &Vec<tiled_json::Tileset>;
//!         tiled_json::Map::tileset_by_gid(&self, gid: u32) -> Option<&Tileset>;
//! 
//! This struct implements the trait HasProperty, which enables easy access of 
//! Tiled properties for maps.  The relevant functions are:
//!     
//!         tiled_json::Map::get_property(&self, name: &str) -> Option<&tiled_json::Property>;
//!         tiled_json::Map::get_property_vector(&self) -> &Vec<tiled_json::Property>;
//!         tiled_json::Map::get_property_value(&self, name: &str) -> Option<&tiled_json::PropertyValue>;
//!         // See the tiled_json::Property struct to see functionality offered.
//! 

use serde::Deserialize;

use crate::color::Color;
use crate::layer::*;
use crate::property::HasProperty;
use crate::property::Property;
use crate::tileset::Tileset;

const MAP_ORTHOGONAL: &str = "orthogonal";
const MAP_ISOMETRIC: &str = "isometric";
const MAP_STAGGERED: &str = "staggered";
const MAP_HEXAGONAL: &str = "hexagonal";

const RENDER_RIGHTDOWN: &str = "right-down";
const RENDER_RIGHTUP: &str = "right-up";
const RENDER_LEFTDOWN: &str = "left-down";
const RENDER_LEFTUP: &str = "left-up";

const STAGGER_ODD: &str = "odd";
const STAGGER_EVEN: &str = "even";
const STAGGER_X: &str = "x";
const STAGGER_Y: &str = "y";

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// The primary structure of all modules.
pub struct Map {
    pub orientation: MapOrientation,

    pub height: u16,
    pub width: u16,

    pub nextobjectid: u32,
    pub nextlayerid: u16,

    pub tileheight: u16,
    pub tilewidth: u16,

    #[serde(default)]
    pub tiledversion: String,

    #[serde(default)]
    pub backgroundcolor: Option<Color>,

    #[serde(default = "default_to_right_down")]
    pub renderorder: RenderOrder,

    #[serde(default)]
    pub hexsidelength: u16,

    #[serde(default)]
    pub staggeraxis: Option<StaggerAxis>,

    #[serde(default)]
    pub staggerindex: Option<StaggerIndex>,

    #[serde(default)]
    pub tilesets: Vec<Tileset>,

    #[serde(default)]
    pub layers: Vec<Layer>,

    #[serde(default)]
    pub properties: Vec<Property>,
}

impl Map {
        /// Borrow the layers vector.
    /// 
    /// This is useful for loading arbitrary game data; you will just iterate through
    /// each layer and utilize the appropriate data.  
    /// 
    /// Most would be more interested in get_layer_by_name().
    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    /// Use this to grab a layer by name.
    /// 
    /// This is particularly useful when you know what kinds of data you need to pick up.
    /// 
    /// Maps might have three static layers describing the scenery; I might name them 
    /// LAYER_GROUND, LAYER_FLOOR, LAYER_SKY.  I could load the map and the tilesets, 
    /// grab those tile layers, and get to building my internal representation.
    pub fn layer_by_name(&self, name: &str) -> Option<&Layer> {
        let name = name.to_string();
        for l in self.layers.iter() {
            if l.name == name {
                return Option::Some(&l);
            }
        }
        Option::None
    }


    /// Borrow the tileset vector. 
    /// 
    /// This is the best way to ensure you have loaded all of the necessary 
    /// textures into memory and are able to decompose gids into their local 
    /// id variants.
    pub fn tilesets(&self) -> &Vec<Tileset> {
        &self.tilesets
    }

    /// Looks for the tileset with the GID provided, automatically stripping
    /// any flags present on the GID (flipping horizontally, vertically, etc...)
    ///
    /// Do note that no bounds checking is done.  If you search for gid: 12352234 and you have
    /// one tileset with 1000 tiles, you will still receive that one tileset back.  Just be sure
    /// the value you send as gid came directly from the map file, tile layer data.
    /// 
    /// This only returns Option::None when no tilesets exist in the map.
    pub fn tileset_by_gid(&self, gid: u32) -> Option<&Tileset> {
        let cf = crate::gid_without_flags(gid);
        for i in self.tilesets.iter().rev() {
            if i.firstgid <= cf {
                return Option::Some(i);
            }
        }
        Option::None
    }


    /// Get the map width in tiles.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Get the map height in tiles.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Get the width of a tile in the map.
    pub fn tile_width(&self) -> u16 {
        self.tilewidth
    }

    /// Get the height of a tile in the map.
    pub fn tile_height(&self) -> u16 {
        self.tileheight
    }

    /// Get the background color of the map if specified.
    pub fn bg_color(&self) -> Option<Color> {
        self.backgroundcolor
    }

    /// Retrieve orientation as variant of MapOrientation.
    /// 
    /// Can be converted to a string using the method to_string().
    pub fn orientation(&self) -> MapOrientation {
        self.orientation
    }

    /// Retrieve renderorder as variant of RenderOrder.
    /// This applies only to Orthogonal Maps but you'll get the default value anyway.  
    /// 
    /// Can be converted to a string using the method to_string().
    pub fn render_order(&self) -> RenderOrder {
        self.renderorder
    }

    /// Get the hex side length. Hex maps only.
    pub fn hex_side_length(&self) -> u16 {
        self.hexsidelength
    }

    /// Get the stagger axis as a variant of StaggerAxis.
    /// Useful only to staggered/hexagonal maps, but will still yeild a 
    /// valid default if you call it irelevantly.
    /// 
    /// Can be converted to a string using the method to_string()
    pub fn stagger_axis(&self) -> Option<StaggerAxis> {
        self.staggeraxis
    }

    /// Get the stagger index as a variant of StaggerIndex.
    /// Useful only to staggered/hexagonal maps, but will still yeild a 
    /// valid default if you call it irelevantly.
    /// 
    /// Can be converted to a string using the method to_string()
    pub fn stagger_index(&self) -> Option<StaggerIndex> {
        self.staggerindex
    }

    /// Get the version of tiled the map was compiled with.
    pub fn tiled_version(&self) -> &String {
        &self.tiledversion
    }
}

impl HasProperty for Map {
    /// Provide access to property values for Maps
    fn get_property_vector(&self) -> &Vec<Property> {
        &self.properties
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(from = "String")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// MapOrientation is an enum with 4 values:
/// - Orthogonal 
/// - Isometric 
/// - Staggered
/// - Hexagonal 
/// 
/// This enum can be turned into the string literal you would find in the Tiled 
/// JSON format via MapOrientation.to_string().
pub enum MapOrientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}

impl std::fmt::Display for MapOrientation {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MapOrientation::Orthogonal => MAP_ORTHOGONAL,
            MapOrientation::Isometric => MAP_ISOMETRIC,
            MapOrientation::Hexagonal => MAP_HEXAGONAL,
            MapOrientation::Staggered => MAP_STAGGERED,
        };
        std::fmt::Display::fmt(s, f)
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(from = "String")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// RenderOrder is an enum with 4 values:
/// - RightDown
/// - RightUp 
/// - LeftDown
/// - LeftUp 
/// 
/// This enum can be turned into the string literal you would find in the Tiled 
/// JSON format via RenderOrder.to_string().  The default is always RightDown.
/// 
/// The renderorder is only applicable when dealing with orthogonal maps.
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl std::fmt::Display for RenderOrder {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RenderOrder::RightDown => RENDER_RIGHTDOWN,
            RenderOrder::LeftDown => RENDER_LEFTDOWN,
            RenderOrder::RightUp => RENDER_RIGHTUP,
            RenderOrder::LeftUp => RENDER_LEFTUP,
        };
        std::fmt::Display::fmt(s, f)
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(from = "String")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// StaggerIndex is an enum with 2 values:
/// - Even
/// - Odd
/// 
/// This enum can be turned into the string literal you would find in the 
/// Tiled JSON format via StaggerIndex.to_string().
/// 
/// This only applies to staggered/hexagonal maps.
pub enum StaggerIndex {
    Even,
    Odd,
}

impl std::fmt::Display for StaggerIndex {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StaggerIndex::Even => STAGGER_EVEN,
            StaggerIndex::Odd => STAGGER_ODD,
        };
        std::fmt::Display::fmt(s, f)
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(from = "String")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// StaggerAxis is an enum with 2 values:
/// - StaggerX
/// - StaggerY
/// 
/// This enum can be turned into the string literal you would find in the 
/// Tiled JSON format via StaggerAxis.to_string().
/// 
/// This only applies to staggered/hexagonal maps.
pub enum StaggerAxis {
    StaggerX,
    StaggerY,
}

impl std::fmt::Display for StaggerAxis {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StaggerAxis::StaggerX => STAGGER_X,
            StaggerAxis::StaggerY => STAGGER_Y,
        };
        std::fmt::Display::fmt(s, f)
    }
}

impl From<String> for MapOrientation {
    fn from(orientation: String) -> Self {
        match orientation.as_str() {
            MAP_ORTHOGONAL => MapOrientation::Orthogonal,
            MAP_ISOMETRIC => MapOrientation::Isometric,
            MAP_STAGGERED => MapOrientation::Staggered,
            MAP_HEXAGONAL => MapOrientation::Hexagonal,
            _ => MapOrientation::Orthogonal,
        }
    }
}

impl From<String> for RenderOrder {
    fn from(order: String) -> Self {
        match order.as_str() {
            RENDER_RIGHTDOWN => RenderOrder::RightDown,
            RENDER_RIGHTUP => RenderOrder::RightUp,
            RENDER_LEFTDOWN => RenderOrder::LeftDown,
            RENDER_LEFTUP => RenderOrder::LeftUp,
            _ => RenderOrder::RightDown,
        }
    }
}

impl From<String> for StaggerIndex {
    fn from(index: String) -> Self {
        match index.as_str() {
            STAGGER_EVEN => StaggerIndex::Even,
            STAGGER_ODD => StaggerIndex::Odd,
            _ => StaggerIndex::Even,
        }
    }
}

impl From<String> for StaggerAxis {
    fn from(axis: String) -> Self {
        match axis.as_str() {
            STAGGER_X => StaggerAxis::StaggerX,
            STAGGER_Y => StaggerAxis::StaggerY,
            _ => StaggerAxis::StaggerX,
        }
    }
}

fn default_to_right_down() -> RenderOrder {
    RenderOrder::RightDown
}
