mod utils;
use glyph_brush_layout::{
    rusttype::{Font, Scale},
    HorizontalAlign as HAlign, VerticalAlign as VAlign, *,
};
use image::{DynamicImage, Rgba};
use js_sys::Uint8Array;
use serde_wasm_bindgen;
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

#[wasm_bindgen(js_name = getBuffer)]
pub fn get_buffer(
    text: &str,
    width: i32,
    height: i32,
    bounds_width: i32,
    bounds_height: i32,
    position_x: i32,
    position_y: i32,
    alignment: Alignment,
    font_size: i32,
    rgb: JsValue,
    font_file: Uint8Array,
) -> Vec<u8> {
    utils::set_panic_hook();
    let rgb_value: (u8, u8, u8) = serde_wasm_bindgen::from_value(rgb).unwrap();
    let fonts = vec![Font::from_bytes(font_file.to_vec()).expect("Error constructing Font")];

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
            screen_position: (position_x as f32, position_y as f32),
            bounds: (bounds_width as f32, bounds_height as f32),
        },
        &[SectionText {
            text: text,
            scale: Scale::uniform(font_size as f32),
            font_id: FontId(0),
            color: [0.0, 1.0, 0.0, 1.0],
        }],
    );

    // Create new RGBA image
    let mut image = DynamicImage::new_rgba8(width as u32, height as u32).to_rgba();

    // Draw glyphs
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.0.pixel_bounding_box() {
            glyph.0.draw(|x, y, v| {
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([rgb_value.0, rgb_value.1, rgb_value.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    return image.to_vec();
}
