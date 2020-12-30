mod utils;
use glyph_brush_layout::{
    rusttype::{Font, Scale},
    HorizontalAlign as HAlign, VerticalAlign as VAlign, *,
};
use image::{DynamicImage, Rgba};
use js_sys::Uint8Array;
// use serde_wasm_bindgen;
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
    pub horizontal: HorizontalAlign,
    pub vertical: VerticalAlign,
}

#[wasm_bindgen(js_name = createAlignment)]
pub fn create_alignment(horizontal: HorizontalAlign, vertical: VerticalAlign) -> Alignment {
    Alignment {
        horizontal,
        vertical,
    }
}

#[wasm_bindgen]
pub struct Dimension(pub i32, pub i32);

#[wasm_bindgen(js_name = createDimension)]
pub fn create_dimension(width: i32, height: i32) -> Dimension {
    Dimension(width, height)
}

#[wasm_bindgen]
pub struct RgbColor(pub u8, pub u8, pub u8);

#[wasm_bindgen(js_name = createRgbColor)]
pub fn create_rgb_color(r: u8, g: u8, b: u8) -> RgbColor {
    RgbColor(r, g, b)
}

#[wasm_bindgen]
pub struct Position(pub i32, pub i32);

#[wasm_bindgen(js_name = createPosition)]
pub fn create_position(x: i32, y: i32) -> Position {
    Position(x, y)
}

#[wasm_bindgen]
pub fn render(
    text: &str,
    text_size: i32,
    text_color: &RgbColor,
    text_font: Uint8Array,
    size: &Dimension,
    bounds: &Dimension,
    position: &Position,
    alignment: &Alignment,
) -> Vec<u8> {
    utils::set_panic_hook();
    let fonts = vec![Font::from_bytes(text_font.to_vec()).expect("Error constructing Font")];

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
    let glyphs = layout.calculate_glyphs(
        &fonts,
        &SectionGeometry {
            screen_position: (position.0 as f32, position.1 as f32),
            bounds: (bounds.0 as f32, bounds.1 as f32),
        },
        &[SectionText {
            text: text,
            scale: Scale::uniform(text_size as f32),
            font_id: FontId(0),
            color: [0.0, 1.0, 0.0, 1.0],
        }],
    );

    // Create new RGBA image
    let mut image = DynamicImage::new_rgba8(size.0 as u32, size.1 as u32).to_rgba();

    // Draw glyphs
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.0.pixel_bounding_box() {
            glyph.0.draw(|x, y, v| {
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([text_color.0, text_color.1, text_color.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    return image.to_vec();
}
