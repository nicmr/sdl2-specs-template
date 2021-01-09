use specs::prelude::*;
use specs_derive::Component;

use sdl2::pixels::Color;

#[derive(Component, Debug, Clone)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x,y}
    }
}

// We can't implement a foreign trait (Component) on a foreign type (SDLColor), so we need to create a wrapper type.
#[derive(Component, Debug, Clone)]
pub struct SDLColor {
    pub color: Color,
}

impl SDLColor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            color: Color::RGB(r, g, b),
        }
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            color: Color::RGBA(r, g, b, a),
        }
    }
}

// Marker struct to designate components controlled by the player
#[derive(Component, Debug, Clone)]
pub struct PlayerControlled {}