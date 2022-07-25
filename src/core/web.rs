use anyhow::{anyhow, Result};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Document, HtmlCanvasElement, Performance, WebGl2RenderingContext, Window};

// Straight taken from https://rustwasm.github.io/book/game-of-life/debugging.html
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
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
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|err| anyhow!("Cannot cast element {:#?} to HtmlCanvasElement", err))
}

pub fn get_webgl2_context(canvas: &HtmlCanvasElement) -> Result<WebGl2RenderingContext> {
    canvas
        .get_context("webgl2")
        .map_err(|err| anyhow!("Error when getting webgl2 context: {:#?}", err))?
        .ok_or_else(|| anyhow!("Cannot find webgl2 context"))?
        .dyn_into::<web_sys::WebGl2RenderingContext>()
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
