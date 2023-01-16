use bevy_ui::{Val, Size, UiRect};
use bevy_render::color::Color;

pub fn size(width: Val, height: Val) -> Size {
    Size::new(width, height)
}

pub fn rect(left: Val, right: Val, top: Val, bottom: Val) -> UiRect {
    UiRect::new(left, right, bottom, top)
}

pub fn all(value: Val) -> UiRect {
    UiRect::all(value)
}

pub fn horozontal(value: Val) -> UiRect {
    UiRect::horizontal(value)
}

pub fn vertical(value: Val) -> UiRect {
    UiRect::vertical(value)
}

pub fn left(value: Val) -> UiRect {
    UiRect::left(value)
}

pub fn right(value: Val) -> UiRect {
    UiRect::right(value)
}

pub fn top(value: Val) -> UiRect {
    UiRect::top(value)
}

pub fn bottom(value: Val) -> UiRect {
    UiRect::bottom(value)
}

pub fn pc(num: impl Tof32) -> Val {
    Val::Percent(num.to_f32())
}

pub fn px(num: impl Tof32) -> Val {
    Val::Px(num.to_f32())
}

pub fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::rgb_u8(r, g, b)
}

pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::rgba_u8(r, g, b, a)
}

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::rgb(r, g, b)
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::rgba(r, g, b, a)
}

pub const fn auto() -> Val {
    Val::Auto
}

pub const fn undefined() -> Val {
    Val::Undefined
}

pub trait Tof32 {
    fn to_f32(self) -> f32;
}

impl Tof32 for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl Tof32 for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i8 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i16 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i128 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u8 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u16 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u128 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}