use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use futures::Future;
use wasm_bindgen::{closure::WasmClosureFnOnce, prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlImageElement, Performance, Response,
    WebGl2RenderingContext, Window,
};

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

macro_rules! debug {
    ( $( $t:tt )* ) => {
        web_sys::console::debug_1(&format!( $( $t )* ).into());
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

pub fn get_2d_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .map_err(|err| anyhow!("Error when getting 2d context: {:#?}", err))?
        .ok_or_else(|| anyhow!("Cannot find 2d context"))?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|err| anyhow!("Cannot cast element {:#?} to CanvasRenderingContext2d", err))
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

pub fn request_animation_loop<F>(mut frame: F) -> Result<i32>
where
    F: FnMut(f64) + 'static,
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |current_time| {
        frame(current_time);
        self::request_animation_frame(
            f.borrow()
                .as_ref()
                .expect("Empty reference to the `requestAnimationFrame` callback"),
        )
        .expect("Cannot run `requestAnimationFrame`");
    }) as Box<dyn FnMut(f64)>));
    let request_id = self::request_animation_frame(
        g.borrow()
            .as_ref()
            .ok_or_else(|| anyhow!("Empty reference to the `requestAnimationFrame` callback"))?,
    )?;
    Ok(request_id)
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

pub fn new_image() -> Result<HtmlImageElement> {
    HtmlImageElement::new().map_err(|err| anyhow!("Cannot create HtmlImageElement: {:#?}", err))
}

pub fn new_canvas(width: u32, height: u32) -> Result<HtmlCanvasElement> {
    let canvas = document()?
        .create_element("canvas")
        .map_err(|err| anyhow!("Cannot create HtmlCanvasElement: {:#?}", err))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|err| anyhow!("Cannot cast element {:#?} to HtmlCanvasElement", err))?;
    canvas.set_width(width);
    canvas.set_height(height);
    Ok(canvas)
}

pub fn closure_once<F, A, R>(f: F) -> Closure<F::FnMut>
where
    F: WasmClosureFnOnce<A, R>,
{
    Closure::once(f)
}

pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(future)
}

pub async fn fetch(uri: &str) -> Result<Response> {
    JsFuture::from(self::window()?.fetch_with_str(uri))
        .await?
        .dyn_into::<Response>()
        .map_err(|err| anyhow!("Error while fetching '{}': {:#?}", uri, err))
}
