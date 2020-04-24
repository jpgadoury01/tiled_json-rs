//! 
//! Objects are accessed usually through Object Group layers.
//! 
//! Objects in Tiled describe just a couple of things:
//! - Text 
//! - Points 
//! - Ellipses 
//! - Polygons
//! - Polylines
//! 
//! How you use these things is up to you.  For example, Tile definitions in 
//! Tilesets contain objects when collision or locations within tiles have been
//! identified in the Tiled editor.  You might use these to describe common paths
//! for NPCs to take or collisions or meshes or text-on-map.
//! 
//! Please see: <https://doc.mapeditor.org/en/stable/reference/json-map-format/#object>
//! 
//! Objects implement the HasProperty trait in order to provide access to
//! Properties.  The relevant functions are:
//!         tiled_json::Object::get_property(&self, name: &str) -> Option<&tiled_json::Property>;
//!         tiled_json::Object::get_property_vector(&self) -> &Vec<tiled_json::Property>;
//!         tiled_json::Object::get_property_value(&self, name: &str) -> Option<&tiled_json::PropertyValue>;
//!         // See the tiled_json::Property struct to see functionality offered.
//! 

use crate::color::Color;
use crate::property::HasProperty;
use crate::property::Property;
use serde::Deserialize;

const ALIGN_LEFT: &str = "left";
const ALIGN_RIGHT: &str = "right";
const ALIGN_JUSTIFY: &str = "justify";
const ALIGN_CENTER: &str = "center";
const ALIGN_TOP: &str = "top";
const ALIGN_BOTTOM: &str = "bottom";

#[derive(Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// Means of describing nodes in objectgroup layers.
pub struct Object {
    pub id: u32,
    pub x: f64,
    pub y: f64,

    #[serde(default)]
    pub gid: Option<u32>, // only if represents tile.

    #[serde(default)]
    pub name: String,

    #[serde(default, rename = "type")]
    pub otype: String,

    #[serde(default)]
    pub height: f64,

    #[serde(default)]
    pub width: f64,

    #[serde(default)]
    pub rotation: f64,

    #[serde(default = "default_to_true")]
    pub visible: bool,

    #[serde(default = "default_to_false")]
    pub ellipse: bool,

    #[serde(default = "default_to_false")]
    pub point: bool,

    #[serde(default)]
    pub polygon: Option<Vec<Point>>,

    #[serde(default)]
    pub polyline: Option<Vec<Point>>,

    #[serde(default)]
    pub text: Option<Text>,

    #[serde(default)]
    pub properties: Vec<Property>,
}

impl HasProperty for Object {
    /// Access properties of Objects.
    fn get_property_vector(&self) -> &Vec<Property> {
        &self.properties
    }
}

impl Object {
    /// Get the object id, unique across all objects.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Horizontal position.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Vertical position.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get object gid if it represents a tile.
    pub fn gid(&self) -> Option<u32> {
        self.gid
    }

    /// Get the name of the object.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the user-defined type of the object.
    pub fn obj_type(&self) -> &String {
        &self.otype
    }

    /// Height of the object
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Width of the object.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Get the rotation of the object.
    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    /// Is the object visible in Tiled.
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Does the object describe an ellipse?
    pub fn is_ellipse(&self) -> bool {
        self.ellipse
    }

    /// Does the object describe a point?
    pub fn is_point(&self) -> bool {
        self.point
    }

    /// Does the object describe a polygon?
    pub fn is_polygon(&self) -> bool {
        self.polygon.is_some()
    }

    /// Does the object describe a polyline?
    pub fn is_polyline(&self) -> bool {
        self.polyline.is_some()
    }

    /// Does the object describe text?
    pub fn is_text(&self) -> bool {
        self.text.is_some()
    }

    /// Get a reference to the polygon vector of points.
    pub fn polygon(&self) -> Option<&Vec<Point>> {
        self.polygon.as_ref()
    }

    /// Gets a reference of the polyline vector of points.
    pub fn polyline(&self) -> Option<&Vec<Point>> {
        self.polyline.as_ref()
    }

    /// Gets a reference to the Text data of the object.
    pub fn text(&self) -> Option<&Text> {
        self.text.as_ref()
    }
}

#[derive(Deserialize, Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// Points describe single points on maps and are generally used to describe 
/// polygons and polylines.  They only have x and y components.
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Get the x value of the point.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the y value of the point.
    pub fn y(&self) -> f64 {
        self.y
    }
}

#[derive(Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// Text is an oject that contains all kinds of characteristics of text that Tiled 
/// is able to display, including the string itself.
pub struct Text {
    pub text: String,

    #[serde(default = "default_to_false")]
    pub bold: bool,

    #[serde(default = "default_to_false")]
    pub italic: bool,

    #[serde(default = "default_to_false")]
    pub strikeout: bool,

    #[serde(default = "default_to_false")]
    pub underline: bool,

    #[serde(default = "default_to_true")]
    pub kerning: bool,

    #[serde(default = "default_to_false")]
    pub wrap: bool,

    #[serde(default = "default_to_16")]
    pub pixelsize: u16,

    #[serde(default = "default_to_black")]
    pub color: Color,

    #[serde(default = "default_to_sansserif")]
    pub fontfamily: String,

    #[serde(default = "default_to_halign_left")]
    pub halign: HAlign,

    #[serde(default = "default_to_valign_top")]
    pub valign: VAlign, // default top
}

impl Text {
    /// Get the string of text needing displayed.
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Is the text supposed to be bold?
    pub fn is_bold(&self) -> bool {
        self.bold
    }

    /// Is the text supposed to be italic?
    pub fn is_italic(&self) -> bool {
        self.italic
    }

    /// Does the text have a striekout?
    pub fn is_strikeout(&self) -> bool {
        self.strikeout
    }

    /// Is the text supposed to be underlined?
    pub fn is_underline(&self) -> bool {
        self.underline
    }

    /// Does the text use kerning for display?
    pub fn is_kerning(&self) -> bool {
        self.kerning
    }

    /// Does the text wrap around?  If it does, the containing Object structure
    /// probably contains more relevant information for wrapping.
    pub fn is_wrap(&self) -> bool {
        self.wrap
    }

    /// Discribe the size in pixels of the text.
    pub fn pixel_size(&self) -> u16 {
        self.pixelsize
    }

    /// Retrieve the color the text is supposed to be.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Get the font-family of the text.
    pub fn font_family(&self) -> &String {
        &self.fontfamily
    }

    /// Get the horizontal alignment in the form of an HAlign enum.
    /// You can call to_string() on the enum if necessary.
    pub fn horizontal_alignment(&self) -> HAlign {
        self.halign
    }

    /// Get the horizontal alignment of the text as a string.
    pub fn halign_as_string(&self) -> String {
        self.halign.to_string()
    }

    /// Get the vertical alignment in the form of a VAlign enum.
    /// You can call to_string() on the enum if necessary.
    pub fn vertical_alignment(&self) -> VAlign {
        self.valign
    }

    /// Get the vertical alignment of the text as a string.
    pub fn valign_as_string(&self) -> String {
        self.valign.to_string()
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(from = "String")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// This enum describes the horizontal alignment of text.  It has 4 variants:
/// - Center 
/// - Right 
/// - Justify 
/// - Left 
/// 
/// to_string() can be called on this enum if required.  
pub enum HAlign {
    Center,
    Right,
    Justify,
    Left,
}

impl std::fmt::Display for HAlign {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HAlign::Center => ALIGN_CENTER,
            HAlign::Right => ALIGN_RIGHT,
            HAlign::Justify => ALIGN_JUSTIFY,
            HAlign::Left => ALIGN_LEFT,
        };
        std::fmt::Display::fmt(s, f)
    }
}

#[derive(Deserialize, Copy, Clone)]
#[serde(from = "String")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// This enum describes the vertical alignment of text.  It has 3 variants:
/// - Center 
/// - Bottom
/// - Top 
/// 
/// to_string() can be called on this enum if required.  
pub enum VAlign {
    Center,
    Bottom,
    Top,
}

impl std::fmt::Display for VAlign {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            VAlign::Center => ALIGN_CENTER,
            VAlign::Bottom => ALIGN_BOTTOM,
            VAlign::Top => ALIGN_TOP,
        };
        std::fmt::Display::fmt(s, f)
    }
}

impl From<String> for HAlign {
    fn from(hal: String) -> Self {
        match hal.as_str() {
            ALIGN_LEFT => HAlign::Left,
            ALIGN_RIGHT => HAlign::Right,
            ALIGN_JUSTIFY => HAlign::Justify,
            ALIGN_CENTER => HAlign::Center,
            _ => HAlign::Left,
        }
    }
}

impl From<String> for VAlign {
    fn from(val: String) -> Self {
        match val.as_str() {
            ALIGN_TOP => VAlign::Top,
            ALIGN_BOTTOM => VAlign::Bottom,
            ALIGN_CENTER => VAlign::Center,
            _ => VAlign::Top,
        }
    }
}

fn default_to_true() -> bool {
    true
}

fn default_to_false() -> bool {
    false
}

fn default_to_16() -> u16 {
    16 as u16
}

fn default_to_black() -> Color {
    Color {
        r: 0,
        b: 0,
        g: 0,
        a: 255,
    }
}

fn default_to_sansserif() -> String {
    "sans-serif".to_string()
}

fn default_to_halign_left() -> HAlign {
    HAlign::Left
}

fn default_to_valign_top() -> VAlign {
    VAlign::Top
}
