//!
//! The Tileset struct contains all of the information necessary for tile 
//! definitions.  
//! 
//! Tilesets contain references to the image, have all of the 
//! relevant information to turn these images into graphical tiles, and include 
//! animation and collision data on a tile-by-tile basis (if any was defined in the 
//! editor, that is).  
//! 
//! The most useful methods of the tileset are the following:
//! 
//!         pub fn coord_by_gid(&self, gid: u32) -> (u16, u16);
//!         pub fn anim_by_gid(&self, gid: u32, milliseconds: u32) -> (u16, u16);
//!         pub fn collision_by_gid(&self, gid: u32) -> Option<&Layer>;
//!         pub fn tile_by_gid(&self, gid: u32) -> Option<&Tile>;
//!         pub fn type_by_gid(&self, gid: u32) -> Option<&String>;
//!         pub fn properties_by_gid(&self, gid: u32) -> Option<&Vec<Property>>;
//! 
//! This struct implements the trait HasProperty, which enables easy access of 
//! Tiled properties for Tilesets.  The relevant functions are:
//!     
//!         tiled_json::Tileset::get_property(&self, name: &str) -> Option<&tiled_json::Property>;
//!         tiled_json::Tileset::get_property_vector(&self) -> &Vec<tiled_json::Property>;
//!         tiled_json::Tileset::get_property_value(&self, name: &str) -> Option<&tiled_json::PropertyValue>;
//!         // See the tiled_json::Property struct to see functionality offered.
//! 
//! See Tiled JSON documentation at:
//! <https://doc.mapeditor.org/en/stable/reference/json-map-format/#tileset>
//! 
use crate::color::Color;
use crate::layer::Layer;
use crate::property::HasProperty;
use crate::property::Property;
use serde::Deserialize;

const ORIENT_ORTHO: &str = "orthogonal";
const ORIENT_ISO: &str = "isometric";

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// The primary means of capturing image data.
pub struct Tileset {
    #[serde(default)]
    pub tiledversion: String,

    pub image: String,
    pub firstgid: u32,

    pub imageheight: u16,
    pub imagewidth: u16,
    pub tileheight: u16,
    pub tilewidth: u16,
    pub tilecount: u32,
    pub columns: u16,

    #[serde(default)]
    pub margin: u16,

    #[serde(default)]
    pub spacing: u16,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub backgroundcolor: Option<Color>,

    #[serde(default)]
    pub transparentcolor: Option<Color>,

    #[serde(default)]
    pub grid: Option<Grid>,

    #[serde(default)]
    pub tiles: Vec<Tile>,

    #[serde(default)]
    pub tileoffset: Option<TileOffset>,

    #[serde(default)]
    pub properties: Vec<Property>,
}

impl Tileset {

    /// This will give you the coordinates in the image of the tile referenced by the gid provided.
    /// You should be sure gid belongs to this tileset.
    /// 
    /// This does not take into account any possible animations that may be on the map.  If you need
    /// animation data, then use anim_by_gid().
    pub fn coord_by_gid(&self, gid: u32) -> (u16, u16) {
        let lid = self.as_local_id(gid);
        let x: u16 = (self.tilewidth + self.spacing) * (lid % self.columns) + self.margin;
        let y: u16 = (self.tileheight + self.spacing) * (lid / self.columns) + self.margin;
        (x, y)
    }

    /// This will give you the coordinates of the tile referenced by the gid provided.  
    /// You should be sure gid belongs to this tileset.
    /// 
    /// You must provided the amount of milliseconds that have passed since creation in order to
    /// get the correct animation frame.
    pub fn anim_by_gid(&self, gid: u32, milliseconds: u32) -> (u16, u16) {
        let mut lid = self.as_local_id(gid);
        for tile in self.tiles.iter() {
            if tile.id == lid {
                let anim = tile.get_anim(milliseconds);
                if anim.0 {
                    lid = anim.1
                }
                break;
            }
        }
        let x: u16 = (self.tilewidth + self.spacing) * (lid % self.columns) + self.margin;
        let y: u16 = (self.tileheight + self.spacing) * (lid / self.columns) + self.margin;
        (x, y)
    }

    /// Tiles may have collision data.  It is named objectgroup in Tiled;
    /// an objectgroup layer defining a collection of objects.
    pub fn collision_by_gid(&self, gid: u32) -> Option<&Layer> {
        let lid = self.as_local_id(gid);
        for tile in self.tiles.iter() {
            if tile.id == lid {
                return tile.objectgroup.as_ref();
            }
        }
        Option::None
    }

    /// Tiles may have user-defined 'types' in Tiled.  Retreive one if it
    /// exists for this gid.
    pub fn type_by_gid(&self, gid: u32) -> Option<&String> {
        let lid = self.as_local_id(gid);
        for tile in self.tiles.iter() {
            if tile.id == lid {
                return tile.ttype.as_ref();
            }
        }
        Option::None
    }

    /// Get a reference to a Tile object if one exists in this tileset by 
    /// the gid of one specified.
    pub fn tile_by_gid(&self, gid: u32) -> Option<&Tile> {
        let lid = self.as_local_id(gid);
        for tile in self.tiles.iter() {
            if tile.id == lid {
                return Option::Some(tile);
            }
        }
        Option::None
    }

    /// Tiles will have their own property lists if defined so in Tiled.  
    /// This function will provide a reference to the property vector if it exists.
    /// If it does not, you may want to refer to the properties of the entire tileset.
    /// 
    /// It may be more convenient to access the property of the tile through
    /// the tile property access methods.  Use them in combination with 
    /// ```Tileset::tile_by_gid(&self, gid: u32)```
    pub fn properties_by_gid(&self, gid: u32) -> Option<&Vec<Property>> {
        let lid = self.as_local_id(gid);
        for tile in self.tiles.iter() {
            if tile.id == lid {
                return Option::Some(tile.get_property_vector());
            }
        }
        Option::None
    }

    /// Get the firstgid of the tileset.
    pub fn first_gid(&self) -> u32 {
        self.firstgid
    }

    /// Get the image of the tileset as a string.
    pub fn image(&self) -> &String {
        &self.image
    }

    /// Image height of the tileset in pixels.
    pub fn image_height(&self) -> u16 {
        self.imageheight
    }

    /// Image width of the tileset is in pixels.
    pub fn image_width(&self) -> u16 {
        self.imagewidth
    }

    /// Get the height of each tile in pixels.
    pub fn tile_height(&self) -> u16 {
        self.tileheight
    }

    /// Get the width of each tile in pixels.
    pub fn tile_width(&self) -> u16 {
        self.tilewidth
    }

    /// Give you the number of tiles in this tileset.
    /// This will include any blank spots in the image.
    pub fn tile_count(&self) -> u32 {
        self.tilecount
    }

    /// Columns refers to the width of the tileset in tile units.
    pub fn columns(&self) -> u16 {
        self.columns
    }

    /// Rows is the height of the tileset in tile units.
    pub fn rows(&self) -> u16 {
        if self.columns == 0 { return 0; }
        (self.tilecount / self.columns as u32) as u16
    }

    /// The distance in pixels from the edge of the image to the start of the tiles.
    /// This border remains constant over all 4 edges of the image.
    pub fn margin(&self) -> u16 {
        self.margin
    }

    /// The distance in pixels from one tile to the next in both axes in the tileset image.
    pub fn spacing(&self) -> u16 {
        self.spacing
    }

    /// The user-defined name of the tileset in Tiled.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The optional background color of the tileset.
    pub fn bg_color(&self) -> Option<Color> {
        self.backgroundcolor
    }

    /// The optional transparent color of the tileset.
    pub fn tp_color(&self) -> Option<Color> {
        self.transparentcolor
    }

    /// An optional grid, that I'm not quite sure what it does.
    pub fn grid(&self) -> Option<Grid> {
        self.grid
    }

    /// This is a vector of any special tiles you have defined in the map editor.
    /// This will include tiles with animations, special properties, objectgroups, etc...
    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    /// Get the tileoffset.  It is an object with an x and a y.
    pub fn tile_offset(&self) -> Option<TileOffset> {
        self.tileoffset
    }

    /// Retrieve the version of Tiled the tileset was compiled with.
    pub fn tiled_version(&self) -> &String {
        &self.tiledversion
    }


    #[inline]
    fn as_local_id(&self, gid: u32) -> u16 {
        (crate::gid_without_flags(gid) - self.firstgid) as u16
    }
}

impl HasProperty for Tileset {
    /// Get access to properties of Tileset.
    fn get_property_vector(&self) -> &Vec<Property> {
        &self.properties
    }
}

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// Tile contains data relevant to overrides of the tileset.
/// This is for containing data specific to certain tiles within the tileset, such
/// animations, collisions, different images, and properties.
///
/// This struct implements the trait HasProperty, which enables easy access of 
/// Tiled properties for Tiles.  The relevant functions are:
///     
///         tiled_json::Tile::get_property(&self, name: &str) -> Option<&tiled_json::Property>;
///         tiled_json::Tile::get_property_vector(&self) -> &Vec<tiled_json::Property>;
///         tiled_json::Tile::get_property_value(&self, name: &str) -> Option<&tiled_json::PropertyValue>;
///         // See the tiled_json::Property struct to see functionality offered.
/// 
pub struct Tile {
    pub id: u16,

    #[serde(default)]
    pub image: Option<String>,

    #[serde(default)]
    pub imageheight: u16,

    #[serde(default)]
    pub imagewidth: u16,

    #[serde(default, rename = "type")]
    pub ttype: Option<String>,

    #[serde(default)]
    pub objectgroup: Option<Layer>,

    #[serde(default)]
    pub animation: Vec<Frame>,

    #[serde(default)]
    pub properties: Vec<Property>,
}

impl HasProperty for Tile {
    /// Get access to the properties of the Tile object.
    fn get_property_vector(&self) -> &Vec<Property> {
        &self.properties
    }
}
 
impl Tile {
    /// This will get you the tileid of the animation frame at msecs (milliseconds) from creation.
    ///
    /// The tuple is a boolean (whether we get an animation or not) and a u16, which will be the local
    /// id of the tile in the parent tileset.
    pub fn get_anim(&self, msecs: u32) -> (bool, u16) {
        if !self.animation.is_empty() {
            let total = self.get_anim_total() as u32;
            let mut msecs = (msecs % total) as u16;
            for f in self.animation.iter() {
                if msecs < f.duration {
                    return (true, f.tileid);
                }
                msecs -= f.duration;
            }
        }
        (false, 0)
    }

    /// id refers to the local id of the tile in the tileset.
    /// This is dissimilar to the gid (global id) used for storing 
    /// data in tile layers.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Take the image height in pixels.
    /// If this was unspecified in Tiled, this data will be 0.
    pub fn image_height(&self) -> u16 {
        self.imageheight
    }

    /// Take the image width in pixels.
    /// If this was unspecified in Tiled, this data will be 0.
    pub fn image_width(&self) -> u16 {
        self.imagewidth
    }

    /// Access the animation data.  This vector can be empty.
    pub fn animation(&self) -> &Vec<Frame> {
        &self.animation
    }

    /// Get the filename of the image this refers to.
    pub fn image(&self) -> Option<&String> {
        self.image.as_ref()
    }

    /// User specified type of tile.
    pub fn get_type(&self) -> Option<&String> {
        self.ttype.as_ref()
    }

    /// This will give you the objectgroup layer.  This is for determining collisions
    /// as defined in the tileset.
    pub fn object_group(&self) -> Option<&Layer> {
        self.objectgroup.as_ref()
    }

    fn get_anim_total(&self) -> u16 {
        let mut total: u16 = 0;
        for f in self.animation.iter() {
            total += f.duration;
        }
        total
    }
}

#[derive(Deserialize, Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// Frame structure describes each moment in an animation.  It has a tileid
/// (the local identifier of the frame in a tileset; NOT a gid) and a duration.
pub struct Frame {
    pub duration: u16,
    pub tileid: u16,
}

impl Frame {
    /// Get the duration of this frame.
    pub fn duration(&self) -> u16 {
        self.duration
    }

    /// Get the local tile id of this frame.  This is not the same as the gid.
    /// The local id describes precisely the location of the tile within a tileset. 
    pub fn tileid(&self) -> u16 {
        self.tileid
    }
}

#[derive(Deserialize, Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// TileOffset structure describes the offset of position for a Tileset.
/// I'm not quite sure how it is used.  It has an x and a y component.
pub struct TileOffset {
    pub x: i32, // in pixels
    pub y: i32, // positive is down
}


#[derive(Deserialize, Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// This will define a custom grid within a tileset.  I'm also not sure how this is
/// used, but it is here in case it is needed.  It has a height, width, and 
/// orientation as GridOrientation.
pub struct Grid {
    pub height: u16,
    pub width: u16,

    #[serde(default = "default_to_orthogonal")]
    pub orientation: GridOrientation,
}

impl Grid {
    /// Get grid width
    pub fn width(self) -> u16 {
        self.width
    }
    /// Get grid height
    pub fn height(self) -> u16 {
        self.height
    }
    /// Get grid orientation as an enum tiled_json::GridOrientation 
    /// that you can call to_string() on.
    pub fn orientation(self) -> GridOrientation {
        self.orientation
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(from = "String")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// GridOrientation is an enum that describes the orientation of a grid.
/// It can be either Orthogonal or Isometric.  
/// 
/// You can call to_string() on this method if need be.
pub enum GridOrientation {
    Orthogonal,
    Isometric,
}

impl std::fmt::Display for GridOrientation {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GridOrientation::Orthogonal => ORIENT_ORTHO,
            GridOrientation::Isometric => ORIENT_ISO,
        };
        std::fmt::Display::fmt(s, f)
    }
}

impl From<String> for GridOrientation {
    fn from(orientation: String) -> Self {
        match orientation.as_str() {
            ORIENT_ORTHO => GridOrientation::Orthogonal,
            ORIENT_ISO => GridOrientation::Isometric,
            _ => GridOrientation::Orthogonal,
        }
    }
}

fn default_to_orthogonal() -> GridOrientation {
    GridOrientation::Orthogonal
}
