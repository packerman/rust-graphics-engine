use anyhow::{anyhow, Result};
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

use crate::{base::web, core::image::Image};

pub struct TextTexture<'a> {
    pub text: &'a str,
    pub width: u32,
    pub height: u32,
    pub font: &'a str,
    pub text_align: &'a str,
    pub text_baseline: &'a str,
    pub font_style: &'a str,
    pub background_style: &'a str,
    pub border_style: &'a str,
    pub border_width: f64,
}

impl Default for TextTexture<'_> {
    fn default() -> Self {
        Self {
            text: "Default text",
            width: 256,
            height: 256,
            font: "32px sans-serif",
            text_align: "center",
            text_baseline: "middle",
            font_style: "black",
            background_style: "white",
            border_style: "red",
            border_width: 1.0,
        }
    }
}

impl<'a> TryFrom<TextTexture<'a>> for Image {
    type Error = anyhow::Error;

    fn try_from(text_texture: TextTexture) -> Result<Self> {
        create_text_canvas(text_texture).map(Image::from)
    }
}

fn create_text_canvas(text_texture: TextTexture) -> Result<HtmlCanvasElement> {
    let canvas = web::new_canvas(text_texture.width, text_texture.height)?;
    let context = web::get_2d_context(&canvas)?;
    context.set_fill_style(&JsValue::from_str(text_texture.background_style));
    context.fill_rect(
        0.0,
        0.0,
        text_texture.width as f64,
        text_texture.height as f64,
    );

    context.set_stroke_style(&JsValue::from_str(text_texture.border_style));
    context.set_line_width(text_texture.border_width);
    context.begin_path();
    context.move_to(0.0, 0.0);
    context.line_to(text_texture.width as f64, 0.0);
    context.line_to(text_texture.width as f64, text_texture.height as f64);
    context.line_to(0.0, text_texture.height as f64);
    context.close_path();
    context.stroke();

    context.set_font(text_texture.font);
    context.set_text_align(text_texture.text_align);
    context.set_text_baseline(text_texture.text_baseline);
    context.set_fill_style(&JsValue::from_str(text_texture.font_style));
    context
        .fill_text(
            text_texture.text,
            text_texture.width as f64 / 2.0,
            text_texture.height as f64 / 2.0,
        )
        .map_err(|err| anyhow!("Error when filling text: {:#?}", err))?;
    Ok(canvas)
}
