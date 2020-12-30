mod utils;
use glyph_brush_layout::{
    rusttype::{Font, Scale},
    HorizontalAlign as HAlign, VerticalAlign as VAlign, *,
};
use image::{DynamicImage, Rgba};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[wasm_bindgen]
pub struct Alignment {
    horizontal: HorizontalAlign,
    vertical: VerticalAlign,
}

#[wasm_bindgen]
impl Alignment {
    #[wasm_bindgen(constructor)]
    pub fn new(horizontal: HorizontalAlign, vertical: VerticalAlign) -> Alignment {
        Alignment {
            horizontal,
            vertical,
        }
    }
}

#[wasm_bindgen]
pub struct Dimension {
    width: i32,
    height: i32,
}

#[wasm_bindgen]
impl Dimension {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Dimension {
        Dimension { width, height }
    }
}

#[wasm_bindgen]
pub struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

#[wasm_bindgen]
impl RgbColor {
    #[wasm_bindgen(constructor)]
    pub fn new(red: u8, green: u8, blue: u8) -> RgbColor {
        RgbColor { red, green, blue }
    }
}

#[wasm_bindgen]
pub struct Position {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[wasm_bindgen]
pub struct Text {
    text: String,
    size: i32,
    color: RgbColor,
    font: Uint8Array,
}

#[wasm_bindgen]
impl Text {
    #[wasm_bindgen(constructor)]
    pub fn new(text: String, size: i32, color: RgbColor, font: Uint8Array) -> Text {
        Text {
            text,
            size,
            color,
            font,
        }
    }
}

#[wasm_bindgen]
pub fn render(
    text: &Text,
    size: &Dimension,
    bounds: &Dimension,
    position: &Position,
    alignment: &Alignment,
) -> Vec<u8> {
    utils::set_panic_hook();
    let fonts = vec![Font::from_bytes(text.font.to_vec()).expect("Error constructing Font")];

    // create aligned layout. we pattern match our wasm-bindgen alignment enums to those of
    // glyph_brush_layout here.
    let layout = Layout::default()
        .h_align(match alignment.horizontal {
            HorizontalAlign::Left => HAlign::Left,
            HorizontalAlign::Center => HAlign::Center,
            HorizontalAlign::Right => HAlign::Right,
        })
        .v_align(match alignment.vertical {
            VerticalAlign::Top => VAlign::Top,
            VerticalAlign::Center => VAlign::Center,
            VerticalAlign::Bottom => VAlign::Bottom,
        });

    // layout glyphs using glyph_brush_layout
    let glyphs = layout.calculate_glyphs(
        &fonts,
        &SectionGeometry {
            screen_position: (position.x as f32, position.y as f32),
            bounds: (bounds.width as f32, bounds.height as f32),
        },
        &[SectionText {
            text: text.text.as_str(),
            scale: Scale::uniform(text.size as f32),
            font_id: FontId(0),
            color: [0.0, 1.0, 0.0, 1.0],
        }],
    );

    // create new RGBA image
    let mut image = DynamicImage::new_rgba8(size.width as u32, size.height as u32).to_rgba();

    // draw glyphs onto image
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.0.pixel_bounding_box() {
            glyph.0.draw(|x, y, v| {
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([
                        text.color.red,
                        text.color.green,
                        text.color.blue,
                        (v * 255.0) as u8,
                    ]),
                )
            });
        }
    }

    // return image
    return image.to_vec();
}
