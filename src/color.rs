//! The Color module might be the only portable structure here.  These are 
//! defined over and over in game code.  
//! 
//! This struct has 4 values: r,g,b,a components describing a color.
//! You can create a new color from ```Color::new(color: &str)``` where ```color```
//! is a string in one of the following formats:
//! 
//!         #rrggbb
//!         #aarrggbb
//! 

use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
#[serde(from = "String")]
#[cfg_attr(debug_assertions, derive(Debug))]
/// The basic structure that describes color across all modules.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Takes a string slice and returns a new Color object.  
    ///
    /// This will fail silently under the impression that this should not be a showstopper--such as when
    /// a string without a leading # symbol is passed or that string has neither 7 nor 9 characters,
    /// or when it contains invalid non-hex characters.
    pub fn new(string: &str) -> Color {
        let length = string.chars().count();
        let mut chars = string.chars();

        if (length != 7 && length != 9) || chars.next() != Option::Some('#') {
            return Color::default_color();
        }

        let a = {
            if length == 9 {
                let a1 = chars.next().unwrap_or('F');
                let a2 = chars.next().unwrap_or('F');
                hex_set_to_u8(a1, a2)
            } else {
                255
            }
        };

        let r = {
            let r1 = chars.next().unwrap_or('F');
            let r2 = chars.next().unwrap_or('F');
            hex_set_to_u8(r1, r2)
        };

        let g = {
            let g1 = chars.next().unwrap_or('F');
            let g2 = chars.next().unwrap_or('F');
            hex_set_to_u8(g1, g2)
        };

        let b = {
            let b1 = chars.next().unwrap_or('F');
            let b2 = chars.next().unwrap_or('F');
            hex_set_to_u8(b1, b2)
        };

        Color { r, g, b, a }
    }

    /// Get a Color object holding the default values when Color::new() gets a string of inappropriate length
    /// or when missing the leading # symbol.
    pub fn default_color() -> Color {
        Color {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    /// Get a String object holding the default values when Color::new() gets a string of inappropriate length
    /// or when missing the leading # symbol.
    pub fn default_color_string() -> String {
        String::from("#FFFF00FF")
    }

    /// Get the red value.
    pub fn red(self) -> u8 {
        self.r
    }
    /// Get the blue value.
    pub fn blue(self) -> u8 {
        self.b
    }
    /// Get the green value.
    pub fn green(self) -> u8 {
        self.g
    }
    /// Get the alpha value.
    pub fn alpha(self) -> u8 {
        self.a
    }
}

impl std::fmt::Display for Color {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity(9);
        s.push('#');
        s.push_str(&u8_to_hex_string(self.a));
        s.push_str(&u8_to_hex_string(self.r));
        s.push_str(&u8_to_hex_string(self.g));
        s.push_str(&u8_to_hex_string(self.b));
        std::fmt::Display::fmt(&*s, f)
    }
}

impl From<String> for Color {
    fn from(cs: String) -> Self {
        Color::new(&cs)
    }
}

fn hex_char_to_u8(hex: char) -> u8 {
    // Forget error-handling.  Improper values will be rare,
    // as will cases where color isn't immediately noticed to be incorrect.
    match hex {
        '0' => 0x00,
        '1' => 0x01,
        '2' => 0x02,
        '3' => 0x03,
        '4' => 0x04,
        '5' => 0x05,
        '6' => 0x06,
        '7' => 0x07,
        '8' => 0x08,
        '9' => 0x09,
        'a' | 'A' => 0x0A,
        'b' | 'B' => 0x0B,
        'c' | 'C' => 0x0C,
        'd' | 'D' => 0x0D,
        'e' | 'E' => 0x0E,
        'f' | 'F' => 0x0F,
        _ => 0x00,
    }
}

fn hex_set_to_u8(hex1: char, hex2: char) -> u8 {
    let v1 = hex_char_to_u8(hex1);
    let v2 = hex_char_to_u8(hex2);
    (v1 << 4) + v2
}

fn u8_to_hex_string(n: u8) -> String {
    let p1 = u8_to_hex_char(n >> 4);
    let p2 = u8_to_hex_char(n & 0x0F);
    let mut s = String::with_capacity(2);
    s.push(p1);
    s.push(p2);
    s
}

fn u8_to_hex_char(n: u8) -> char {
    match n {
        0x00 => '0',
        0x01 => '1',
        0x02 => '2',
        0x03 => '3',
        0x04 => '4',
        0x05 => '5',
        0x06 => '6',
        0x07 => '7',
        0x08 => '8',
        0x09 => '9',
        0x0A => 'A',
        0x0B => 'B',
        0x0C => 'C',
        0x0D => 'D',
        0x0E => 'E',
        0x0F => 'F',
        _ => '0',
    }
}
