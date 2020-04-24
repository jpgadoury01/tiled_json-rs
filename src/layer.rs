//! The Layer struct contains all of the necessary information required to build
//! a layer of any kind.  
//! 
//! There are four types of layers:
//! - Tile layers
//! - Object groups
//! - Image layers
//! - Groups
//! 
//! See relevant data and structure information at:
//! > <https://doc.mapeditor.org/en/stable/reference/json-map-format/#layer>  
//! 
//! Each is contained within its own enum LayerDataContainer accessible through 
//! a field of Layer called layerdata.  LayerDataContainer has 4 variants, each
//! with its own set of relevant variables:
//! - LayerDataContainer::TileLayer
//! 
//!         data: Vec<u32>
//! - LayerDataContainer::ObjectGroup
//! 
//!         draworder: tiled_json::DrawOrder
//!         objects:   Vec<tiled_json::Object>
//! - LayerDataContainer::ImageLayer
//! 
//!         image:            String
//!         transparentcolor: Option<tiled_json::Color>
//! - LayerDataContainer::Group
//! 
//!         layers: Vec<tiled_json::Layer>
//! 
//! This layer class provides a number of convenience functions to enable ease of 
//! working with the enum structures provided.  It is not unreasonable to assume
//! that you will know the type of layer you are accessing, so I've included the
//! following functions to reduce verbosity:
//! 
//!         tiled_json::Layer::is_tile_layer(&self) -> bool;
//!         tiled_json::Layer::is_object_group(&self) -> bool;
//!         tiled_json::Layer::is_image_layer(&self) -> bool;
//!         tiled_json::Layer::is_group(&self) -> bool;
//! 
//!         // Get tile layer data if self is a tile layer.
//!         tiled_json::Layer::get_data(&self) -> Option<&Vec<u32>>;
//! 
//!         // The following get object group data if layer refers to an object group:
//!         tiled_json::Layer::get_draworder(&self) -> Option<DrawOrder>;
//!         tiled_json::Layer::get_objects_vector(&self) -> Option<&Vec<Object>>;
//! 
//!         // The following get image layer data if the layer refers to an image layer:
//!         tiled_json::Layer::get_image(&self) -> Option<&String>;
//!         tiled_json::Layer::get_transparentcolor(&self) -> Option<Color>;
//! 
//!         // Get group data if the layer refers to a group of layers.
//!         tiled_json::Layer::get_layers(&self) -> Option<&Vec<Layer>>;
//! 
//! This struct implements the trait HasProperty, which enables easy access of 
//! Tiled properties for layers.  The relevant functions are:
//!     
//!         tiled_json::Layer::get_property(&self, name: &str) -> Option<&tiled_json::Property>;
//!         tiled_json::Layer::get_property_vector(&self) -> &Vec<tiled_json::Property>;
//!         tiled_json::Layer::get_property_value(&self, name: &str) -> Option<&tiled_json::PropertyValue>;
//!         // See the tiled_json::Property struct to see functionality offered.
//! 

use crate::color::Color;
use crate::layerreader::LayerReader;
use crate::object::Object;
use crate::property::HasProperty;
use crate::property::Property;
use serde::Deserialize;

pub const DRAWORDER_TOPDOWN: &str = "topdown";
pub const DRAWORDER_INDEX: &str = "index";

pub const LAYER_TILE: &str = "tilelayer";
pub const LAYER_OBJGROUP: &str = "objectgroup";
pub const LAYER_IMAGE: &str = "imagelayer";
pub const LAYER_GROUP: &str = "group";

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Deserialize)]
#[serde(try_from = "LayerReader")]
/// The primary method of describing nodes in maps.
pub struct Layer {
    pub id: Option<u32>,
    pub name: String,

    pub opacity: f64,
    pub visible: bool,
    pub width: u32,
    pub height: u32,
    pub offsetx: f64,
    pub offsety: f64,

    #[serde(rename = "type")]
    pub ltype: LayerType,

    pub layerdata: LayerDataContainer,
    pub properties: Vec<Property>,
}

impl HasProperty for Layer {
    /// Enable property access for layers.
    fn get_property_vector(&self) -> &Vec<Property> {
        &self.properties
    }
}

impl Layer {
    /// Determines if the item at pos is flipped along the horizontal axis.
    /// 
    /// Only appropriate for tile layers.  If pos exists outside of tile layer data,
    /// false will be returned.  This is used during iteration over tile layer data; 
    /// to check by the gid directly, please use 
    /// tiled_json::gid_flipped_horizontally(gid: u32).
    pub fn is_flipped_horizontally(&self, pos: usize) -> bool {
        if let LayerDataContainer::TileLayer { ref data } = self.layerdata {
            if data.len() < pos {
                return crate::gid_flipped_horizontally(data[pos]);
            }
        }
        false
    }

    /// Determines if the item at pos is flipped along the vertical axis.
    /// 
    /// Only appropriate for tile layers.  If pos exists outside of tile layer data,
    /// false will be returned.  This is used during iteration over tile layer data; 
    /// to check by the gid directly, please use 
    /// tiled_json::gid_flipped_vertically(gid: u32).
    pub fn is_flipped_vertically(&self, pos: usize) -> bool {
        if let LayerDataContainer::TileLayer { ref data } = self.layerdata {
            if data.len() < pos {
                return crate::gid_flipped_vertically(data[pos]);
            }
        }
        false
    }

    /// Determines if the item at pos is flipped diagonally.
    /// 
    /// Only appropriate for tile layers.  If pos exists outside of tile layer data,
    /// false will be returned.  This is used during iteration over tile layer data; 
    /// to check by the gid directly, please use 
    /// tiled_json::gid_flipped_diagonally(gid: u32).
    pub fn is_flipped_diagonally(&self, pos: usize) -> bool {
        if let LayerDataContainer::TileLayer { ref data } = self.layerdata {
            if data.len() < pos {
                return crate::gid_flipped_diagonally(data[pos]);
            }
        }
        false
    }

    /// Use this function if you need to check all axes at the same time.
    /// - result.0 = horizontally flipped
    /// - result.1 = vertically flipped
    /// - result.2 = diagonally flipped 
    /// 
    /// Only appropriate for tile layers.  If pos exists outside of tile layer data,
    /// false will be returned.  This is used during iteration over tile layer data; 
    /// to check by the gid directly, please use tiled_json::gid_flipped_hvd(gid: u32).
    pub fn is_flipped_hvd(&self, pos: usize) -> (bool, bool, bool) {
        if let LayerDataContainer::TileLayer { ref data } = self.layerdata {
            if data.len() < pos {
                return crate::gid_flipped_hvd(data[pos]);
            }
        }
        (false, false, false)
    }

    /// Retrieve the gid at the pos submitted without any of the flags present.
    /// 
    /// Only appropriate for tile layers.  If pos exists outside of tile layer data,
    /// false will be returned.  This is used during iteration over tile layer data; 
    /// to check by the gid directly, please use tiled_json::gid_without_flags(gid: u32).
    pub fn get_gid_without_flags(&self, pos: usize) -> u32 {
        if let LayerDataContainer::TileLayer { ref data } = self.layerdata {
            if data.len() < pos {
                return crate::gid_without_flags(data[pos]);
            }
        }
        0
    }

    /// See if the layer is a tile layer.
    pub fn is_tile_layer(&self) -> bool {
        match self.ltype {
            LayerType::TileLayer => true,
            _ => false,
        }
    }

    /// See if the layer is an object group.
    pub fn is_object_group(&self) -> bool {
        match self.ltype {
            LayerType::ObjectGroup => true,
            _ => false,
        }
    }

    /// See if the layer is an image layer.
    pub fn is_image_layer(&self) -> bool {
        match self.ltype {
            LayerType::ImageLayer => true,
            _ => false,
        }
    }

    /// See if the layer is a group of layers.
    pub fn is_group(&self) -> bool {
        match self.ltype {
            LayerType::Group => true,
            _ => false,
        }
    }

    /// This is a shortcut method to get borrowed tile data of a Tile Layer.
    /// It will return None if this layer is not a TileLayer.
    pub fn get_data(&self) -> Option<&Vec<u32>> {
        if let LayerDataContainer::TileLayer { data: ref x } = self.layerdata {
            Option::Some(x)
        } else {
            Option::None
        }
    }

    /// A shortcut method to get the draworder of an objgroup layer.
    /// it will return None if the layer is not an ObjGroup layer.
    pub fn get_draworder(&self) -> Option<DrawOrder> {
        if let LayerDataContainer::ObjectGroup {
            draworder: order, ..
        } = self.layerdata
        {
            Option::Some(order)
        } else {
            Option::None
        }
    }

    /// A shortcut method to borrow the Vector of Objects in an ObjectGroup layer.
    /// This will return None if the layer is not an ObjectGroup layer.
    pub fn get_objects_vector(&self) -> Option<&Vec<Object>> {
        if let LayerDataContainer::ObjectGroup {
            objects: ref obj, ..
        } = self.layerdata
        {
            Option::Some(obj)
        } else {
            Option::None
        }
    }

    /// A shortcut method to borrow the image string of an ImageLayer.
    /// You will get None back if self doesn't reference an ImageLayer.
    pub fn get_image(&self) -> Option<&String> {
        if let LayerDataContainer::ImageLayer { image: ref im, .. } = self.layerdata {
            Option::Some(im)
        } else {
            Option::None
        }
    }

    /// A shortcut method to borrow the transparent color of an ImageLayer.
    /// You will get None back if self doesn't reference an ImageLayer.
    pub fn get_transparentcolor(&self) -> Option<Color> {
        if let LayerDataContainer::ImageLayer {
            transparentcolor: tc,
            ..
        } = self.layerdata
        {
            tc.clone()
        } else {
            Option::None
        }
    }

    /// This is a shortcut function for GROUP layers.  It will return the underlying vector of layers
    /// that are probably more relevant than the group itself.
    /// This will only return None if self is not describing a Group layer, even if the vector is empty.
    pub fn get_layers(&self) -> Option<&Vec<Layer>> {
        if let LayerDataContainer::Group { layers: ref lays } = self.layerdata {
            Option::Some(lays)
        } else {
            Option::None
        }
    }

    /// Get the x value of the layer (always 0).
    pub fn x(&self) -> u32 {
        0
    }

    /// Get the y value of the layer (always 0).
    pub fn y(&self) -> u32 {
        0
    }

    /// Not all layers have unique ids.  The layers present as Tile::objectgroups do not have them, and
    /// there may be others without.
    pub fn id(&self) -> Option<u32> {
        self.id
    }

    /// Get the name of the layer.  It is an empty string unless specified in Tiled editor.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// This will be 1.0 if no value was found.
    pub fn opacity(&self) -> f64 {
        self.opacity
    }

    /// Is this visible in the editor?
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// get width in tiles!  same as mapwidth since only fixed maps are supported.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// get height in tiles!  same as mapheight since only fixed maps are supported.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// horizontal layer offset in pixels (beyond my comprehension: its a float)
    pub fn offset_x(&self) -> f64 {
        self.offsetx
    }

    /// vertical layer offset in pixels (beyond my comprehension: its a float)
    pub fn offset_y(&self) -> f64 {
        self.offsety
    }

    /// Get the Layer Type: one of LayerType::{Tile Layer, ObjectGroup, ImageLayer, Group}
    pub fn layer_type(&self) -> LayerType {
        self.ltype
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
/// The LayerDataContainer is an enum that describes the four different types of 
/// layers that can be present within a map.  You can access these values
/// directly if need be, but I have included layer methods that will retrieve 
/// this data without the need to resolve the enum yourself.  The code can be 
/// quite verbose when working with namespaces and identifiers this large.
pub enum LayerDataContainer {
    TileLayer {
        data: Vec<u32>,
    },
    ObjectGroup {
        draworder: DrawOrder,
        objects: Vec<Object>,
    },
    ImageLayer {
        image: String,
        transparentcolor: Option<Color>,
    },
    Group {
        layers: Vec<Layer>,
    },
}

#[derive(Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// LayerType telling us the type of Layer this is.  This is used more interally
/// than anything else.  You can call to_string() on this enum.
pub enum LayerType {
    TileLayer,
    ObjectGroup,
    ImageLayer,
    Group,
}

impl std::fmt::Display for LayerType {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LayerType::TileLayer => LAYER_TILE,
            LayerType::ObjectGroup => LAYER_OBJGROUP,
            LayerType::ImageLayer => LAYER_IMAGE,
            LayerType::Group => LAYER_GROUP,
        };
        std::fmt::Display::fmt(s, f)
    }
}

#[derive(Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// The DrawOrder for the layer.  This is only used on Object Group layers.
/// You can call to_string() on variants of this enum.
pub enum DrawOrder {
    TopDown,
    Index,
}

impl std::fmt::Display for DrawOrder {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DrawOrder::TopDown => DRAWORDER_TOPDOWN,
            DrawOrder::Index => DRAWORDER_INDEX,
        };
        std::fmt::Display::fmt(s, f)
    }
}

