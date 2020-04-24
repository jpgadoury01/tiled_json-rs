//! 
//! Properties is a structure designed for describing all the different kinds of 
//! properties that Tiled allows one to describe.  
//! 
//! They can be attached to maps,
//! layers, tilesets, tiles, and objects and this makes them incredibly useful for
//! describing arbitrary data within the map itself. 
//! 
//! Each property describes:
//!         Files   Strings   Integers
//!         Floats  Booleans  Colors
//!
//! In this library, each Property contains a name and a PropertyValue.
//! PropertyValue is an enum variant that contains the data respective to the type.
//! 
//! [It is useful to note that should PropertyValue fail to serialize from the map,
//! it will default to string and any related data will be stored inside.]
//! 
//! The relevant functions here are:
//! 
//!         Property::get_string(&self) -> Option<&String>;
//!         Property::get_int(&self) -> Option<i32>;
//!         Property::get_float(&self) -> Option<f64>;
//!         Property::get_bool(&self) -> Option<bool>;
//!         Property::get_color(&self) -> Option<Color>;
//!  
//! Anything that has a properties value will implement ```HasProperty``` which 
//! enables a number of convenience functions to facilitate property access.  
//!         
//!         ::get_property(&self, name: &str) -> Option<&tiled_json::Property>;
//!         ::get_property_vector(&self) -> &Vec<tiled_json::Property>;
//!         ::get_property_value(&self, name: &str) -> Option<&tiled_json::PropertyValue>;
//! 

use crate::color::Color;
use serde::Deserialize;

const TYPE_FILE: &str = "file";
const TYPE_STRING: &str = "string";
const TYPE_INT: &str = "int";
const TYPE_FLOAT: &str = "float";
const TYPE_BOOL: &str = "bool";
const TYPE_COLOR: &str = "color";

#[derive(Deserialize, Clone)]
#[serde(from = "PropertyLoader")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// The structure defining all properties and how to use them.
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
#[cfg_attr(debug_assertions, derive(Debug))]
/// This is the power behind the Property struct.  Each variant describes a 
/// different data type. 
/// - StringV describes a string.
/// - Int describes a signed integer.
/// - Float describes a floating point number.
/// - Bool describes a boolean.
/// - Color describes a Color object.
/// - File describes a file in string format (the name)
pub enum PropertyValue {
    StringV(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    Color(Color),
    File(String),
}

impl Property {
    /// Get the name of the property.  Returns a simple string.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the type of the property as a string slice.
    pub fn type_as_string(&self) -> &'static str {
        match self.value {
            PropertyValue::Int(_) => TYPE_INT,
            PropertyValue::Float(_) => TYPE_FLOAT,
            PropertyValue::Bool(_) => TYPE_BOOL,
            PropertyValue::Color(_) => TYPE_COLOR,
            PropertyValue::StringV(_) => TYPE_STRING,
            PropertyValue::File(_) => TYPE_FILE,
        }
    }

    /// Get a reference to the underlying PropertyValue enum should you wish to match on it
    /// manually.  
    ///
    /// Since we should know what the property type is by name, there are better
    /// methods of obtaining the appropriate data. See get_string(), get_int(), get_float(), etc...
    pub fn get_pvalue(&self) -> &PropertyValue {
        &self.value
    }

    /// Provides the string data if this proprty is a string, returns Option::None otherwise.
    pub fn get_string(&self) -> Option<&String> {
        if let PropertyValue::StringV(ref s) = self.value {
            Option::Some(s)
        } else {
            Option::None
        }
    }

    /// Provides the integer data if this property is an integer, or Option::None.
    pub fn get_int(&self) -> Option<i32> {
        if let PropertyValue::Int(i) = self.value {
            Option::Some(i)
        } else {
            Option::None
        }
    }

    /// Provides the float data if this property is an float, or Option::None.
    pub fn get_float(&self) -> Option<f64> {
        if let PropertyValue::Float(f) = self.value {
            Option::Some(f)
        } else {
            Option::None
        }
    }

    /// Provides the boolean data if this property is a boolean, or Option::None.
    pub fn get_bool(&self) -> Option<bool> {
        if let PropertyValue::Bool(b) = self.value {
            Option::Some(b)
        } else {
            Option::None
        }
    }

    /// Provides the Color object if this property describes a color, or Option::None.
    pub fn get_color(&self) -> Option<Color> {
        if let PropertyValue::Color(c) = self.value {
            Option::Some(c)
        } else {
            Option::None
        }
    }

    /// Provides the filename if this property describes a file, or Option::None.
    pub fn get_file(&self) -> Option<&String> {
        if let PropertyValue::File(ref s) = self.value {
            Option::Some(s)
        } else {
            Option::None
        }
    }
}

pub trait HasProperty {
    /// Get the whole Property vector.
    fn get_property_vector(&self) -> &Vec<Property>;

    /// Find a property by name.
    fn get_property(&self, name: &str) -> Option<&Property> {
        let pv = self.get_property_vector();
        for prop in pv.iter() {
            if prop.name().as_str() == name {
                return Option::Some(prop);
            }
        }
        Option::None
    }

    /// Get the PropertyValue by name from an obhect that has a list of properties.
    ///
    /// One might be better served using get_property() and using the property functions from
    /// there.
    fn get_property_value(&self, name: &str) -> Option<&PropertyValue> {
        let pv = self.get_property(name);
        if let Option::Some(x) = pv {
            return Option::Some(x.get_pvalue());
        }
        Option::None
    }
}

#[derive(Deserialize)]
struct PropertyLoader {
    name: String,
    #[serde(rename = "type")]
    ptype: String,
    value: PropertyValue, // based on type
}

impl From<PropertyLoader> for Property {
    fn from(pl: PropertyLoader) -> Self {
        let v = match pl.value {
            PropertyValue::Bool(x) => PropertyValue::Bool(x),
            PropertyValue::Float(x) => PropertyValue::Float(x),
            PropertyValue::Color(x) => PropertyValue::Color(x),

            PropertyValue::Int(x) => {
                if pl.ptype == TYPE_FLOAT {
                    PropertyValue::Float(x as f64)
                } else {
                    PropertyValue::Int(x)
                }
            }

            PropertyValue::StringV(x) | PropertyValue::File(x) => {
                if pl.ptype == TYPE_STRING {
                    PropertyValue::StringV(x)
                } else if pl.ptype == TYPE_FILE {
                    PropertyValue::File(x)
                } else {
                    PropertyValue::Color(Color::new(&x))
                }
            }
        };
        Property {
            name: pl.name,
            value: v,
        }
    }
}
