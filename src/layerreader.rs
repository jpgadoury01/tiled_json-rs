use base64;
use flate2::bufread::GzDecoder;
use flate2::bufread::ZlibDecoder;
use std::io::Read;

use serde::export::TryFrom;
use serde::Deserialize;

use crate::color::Color;
use crate::layer::*;
use crate::object::Object;
use crate::property::Property;

#[derive(Deserialize)]
pub struct LayerReader {
    #[serde(default)]
    height: u32,

    #[serde(default)]
    width: u32,

    #[serde(rename = "type")]
    ltype: String,

    #[serde(default)]
    id: Option<u32>,

    #[serde(default)]
    name: String,

    #[serde(default)]
    compression: Option<String>,

    #[serde(default)]
    offsetx: f64,

    #[serde(default)]
    offsety: f64,

    #[serde(default = "default_to_one_f64")]
    opacity: f64,

    #[serde(default = "default_to_true")]
    visible: bool,

    #[serde(default)]
    transparentcolor: Option<Color>,

    #[serde(default)]
    draworder: Option<String>,

    #[serde(default)]
    image: Option<String>,

    #[serde(default)]
    data: Option<TileLayerDataReader>,

    #[serde(default)]
    layers: Option<Vec<Layer>>,

    #[serde(default)]
    objects: Option<Vec<Object>>,

    #[serde(default)]
    properties: Option<Vec<Property>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TileLayerDataReader {
    Vector(Vec<u32>),
    Base64(String),
}

impl TryFrom<LayerReader> for Layer {
    type Error = String;

    fn try_from(lr: LayerReader) -> Result<Self, Self::Error> {
        let ltype: LayerType;
        let layerdata: LayerDataContainer;

        match lr.ltype.as_str() {
            LAYER_TILE => {
                ltype = LayerType::TileLayer;
                let data = get_tile_layer_data(
                    lr.data,
                    (lr.width * lr.height) as usize,
                    &lr.name,
                    &lr.compression,
                )?;
                layerdata = LayerDataContainer::TileLayer { data };
            }

            LAYER_OBJGROUP => {
                ltype = LayerType::ObjectGroup;
                let draworder = match lr.draworder.unwrap_or_default().as_str() {
                    DRAWORDER_INDEX => DrawOrder::Index,
                    DRAWORDER_TOPDOWN => DrawOrder::TopDown,
                    _ => DrawOrder::TopDown,
                };
                layerdata = LayerDataContainer::ObjectGroup {
                    draworder,
                    objects: lr.objects.unwrap_or_default(),
                };
            }

            LAYER_IMAGE => {
                ltype = LayerType::ImageLayer;
                let image = lr.image.unwrap_or_default();
                let transparentcolor = lr.transparentcolor;
                layerdata = LayerDataContainer::ImageLayer {
                    image,
                    transparentcolor,
                };
            }

            LAYER_GROUP => {
                ltype = LayerType::Group;
                layerdata = LayerDataContainer::Group {
                    layers: lr.layers.unwrap_or_default(),
                };
            }
            _ => {
                return Err(format!(
                    "invalid layer type {} (id: {}, name: {})",
                    lr.ltype,
                    if let Option::Some(x) = lr.id {
                        x.to_string()
                    } else {
                        "nil".to_string()
                    },
                    lr.name
                ))
            }
        };

        let id = lr.id;
        let name = lr.name;
        let opacity = lr.opacity;
        let visible = lr.visible;
        let width = lr.width;
        let height = lr.height;
        let offsetx = lr.offsetx;
        let offsety = lr.offsety;
        let properties = lr.properties.unwrap_or_default();

        Ok(Self {
            id,
            name,
            opacity,
            visible,
            width,
            height,
            offsetx,
            offsety,
            ltype,
            layerdata,
            properties,
        })
    }
}

fn get_tile_layer_data(
    data: Option<TileLayerDataReader>,
    size: usize,
    name: &str,
    compression: &Option<String>,
) -> Result<Vec<u32>, String> {
    // The point here is to fail only when base64 decoding and decompression fail.
    // In the event no data is read, we simply return an empty vector.
    match data {
        // For CSV types
        Option::Some(TileLayerDataReader::Vector(v)) => Ok(v),
        // For failed reads:
        Option::None => Ok(Vec::<u32>::new()),
        // For String AKA base64 and possibly compressed.
        Option::Some(TileLayerDataReader::Base64(s)) => {
            let v = decode_tile_layer_data(&s, size, name, compression)?;
            Ok(v.unwrap_or_default())
        }
    }
}

fn decode_tile_layer_data(
    string_data: &str,
    size: usize,
    name: &str,
    compression: &Option<String>,
) -> Result<Option<Vec<u32>>, String> {
    let size_bytes = size * 4;
    let decoded = base64::decode(string_data);
    if decoded.is_err() {
        return Err(format!(
            "Cannot decode base64 string of tilelayer named: {}",
            name
        ));
    }

    // Shadow old decoded, not needed anymore.
    let mut decoded = decoded.unwrap();
    let mut decompressed: Vec<u8> = Vec::with_capacity(size_bytes);
    let mut vector: &mut Vec<u8> = &mut decoded;

    if let Option::Some(c) = compression {
        if !c.is_empty() {
            if !decompress_tile_layer_data(&decoded, &mut decompressed, c) {
                return Err(format!(
                    "invalid compression data in tilelayer named {}, compression: {}",
                    name, c
                ));
            }
            vector = &mut decompressed;
        }
    }

    if vector.is_empty() {
        return Ok(Option::None);
    }

    if vector.len() != size_bytes {
        return Err(format!("corrupted tilelayer data for name: {}", name));
    }

    let mut ret: Vec<u32> = Vec::with_capacity(size);
    let mut x: usize = 0;
    while x < (size_bytes - 3) {
        ret.push(
            (vector[x] as u32)
                | ((vector[x + 1] as u32) << 8)
                | ((vector[x + 2] as u32) << 16)
                | ((vector[x + 3] as u32) << 24),
        );
        x += 4;
    }
    Ok(Option::Some(ret))
}

fn decompress_tile_layer_data(
    decoded: &[u8],
    mut decompressed: &mut Vec<u8>,
    compression: &str,
) -> bool {
    match compression {
        "zlib" => {
            let mut zl = ZlibDecoder::new(&decoded[..]);
            if zl.read_to_end(&mut decompressed).is_err() {
                return false;
            }
        }
        "gzip" => {
            let mut gz = GzDecoder::new(&decoded[..]);
            if gz.read_to_end(&mut decompressed).is_err() {
                return false;
            }
        }
        _ => return false,
    };
    true
}


fn default_to_one_f64() -> f64 {
    1.0 as f64
}

fn default_to_true() -> bool {
    true
}
