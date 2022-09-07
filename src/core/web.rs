use anyhow::{anyhow, Result};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Document, HtmlCanvasElement, Performance, WebGl2RenderingContext, Window};

// Straight taken from https://rustwasm.github.io/book/game-of-life/debugging.html
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("Cannot find window"))
}

pub fn document() -> Result<Document> {
    window().and_then(|window| {
        window
            .document()
            .ok_or_else(|| anyhow!("Cannot find document"))
    })
}

pub fn get_canvas_by_id(id: &str) -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id(id)
        .ok_or_else(|| anyhow!("Cannot find element with id {:#?}", id))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|err| anyhow!("Cannot cast element {:#?} to HtmlCanvasElement", err))
}

pub fn get_webgl2_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext> {
    canvas
        .get_context("webgl2")
        .map_err(|err| anyhow!("Error when getting webgl2 context: {:#?}", err))?
        .ok_or_else(|| anyhow!("Cannot find webgl2 context"))?
        .dyn_into::<WebGl2RenderingContext>()
        .map_err(|err| anyhow!("Cannot cast element {:#?} to WebGl2RenderingContext", err))
}

pub fn performance() -> Result<Performance> {
    window().and_then(|window| {
        window
            .performance()
            .ok_or_else(|| anyhow!("Cannot find performance"))
    })
}

pub fn now() -> Result<f64> {
    performance().map(|perf| perf.now())
}

pub fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) -> Result<i32> {
    window().and_then(|window| {
        window
            .request_animation_frame(f.as_ref().unchecked_ref())
            .map_err(|err| anyhow!("Cannot register requestAnimationFrame: {:#?}", err))
    })
}

pub fn window_inner_size(window: &Window) -> Result<(f64, f64)> {
    Ok((
        window
            .inner_width()
            .map_err(|err| anyhow!("Error when getting window inner width: {:#?}", err))?
            .as_f64()
            .ok_or_else(|| anyhow!("Cannot cast width to f64"))?,
        window
            .inner_height()
            .map_err(|err| anyhow!("Error when getting window inner height: {:#?}", err))?
            .as_f64()
            .ok_or_else(|| anyhow!("Cannot cast width to f64"))?,
    ))
}

pub fn canvas_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    (canvas.width(), canvas.height())
}

pub fn set_canvas_size(canvas: &HtmlCanvasElement, size: (u32, u32)) {
    canvas.set_width(size.0);
    canvas.set_height(size.1);
}

pub fn get_canvas(context: &WebGl2RenderingContext) -> Result<HtmlCanvasElement> {
    context
        .canvas()
        .ok_or_else(|| anyhow!("Cannot find canvas"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|err| anyhow!("Cannot cast element {:#?} to WebGl2RenderingContext", err))
}
