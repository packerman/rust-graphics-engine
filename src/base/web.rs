use std::{cell::RefCell, rc::Rc, sync::Mutex};

use anyhow::{anyhow, Result};
use futures::{channel::oneshot, Future};
use js_sys::ArrayBuffer;
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

macro_rules! warn {
    ( $( $t:tt )* ) => {
        web_sys::console::warn_1(&format!( $( $t )* ).into())
    }
}

macro_rules! error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

macro_rules! debug {
    ( $( $t:tt )* ) => {
        web_sys::console::debug_1(&format!( $( $t )* ).into())
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

pub fn canvas_size(canvas: &HtmlCanvasElement) -> (u32, u32) {
    (canvas.width(), canvas.height())
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

async fn fetch(uri: &str) -> Result<Response> {
    JsFuture::from(self::window()?.fetch_with_str(uri))
        .await
        .map_err(|err| anyhow!("Error while fetching '{}': {:#?}", uri, err))?
        .dyn_into::<Response>()
        .map_err(|err| anyhow!("Error while casting {} to Response: {:#?}", uri, err))
}

pub async fn fetch_json(uri: &str) -> Result<JsValue> {
    let response = self::fetch(uri).await?;
    JsFuture::from(
        response
            .json()
            .map_err(|err| anyhow!("Error while fetching Response from {}: {:#?}", uri, err))?,
    )
    .await
    .map_err(|err| anyhow!("Error while fetching JSON from {}: {:#?}", uri, err))
}

pub async fn fetch_array_buffer(uri: &str) -> Result<ArrayBuffer> {
    let array_buffer = self::fetch(uri)
        .await?
        .array_buffer()
        .map_err(|err| anyhow!("Error while fetching ArrayBuffer from {}: {:#?}", uri, err))?;
    JsFuture::from(array_buffer)
        .await
        .map_err(|err| anyhow!("Error while fetching ArrayBuffer from {}: {:#?}", uri, err))?
        .dyn_into()
        .map_err(|err| anyhow!("Error while fetching ArrayBuffer from {}: {:#?}", uri, err))
}

pub async fn fetch_image(uri: &str) -> Result<HtmlImageElement> {
    let image = self::new_image()?;
    let (sender, receiver) = oneshot::channel::<Result<()>>();
    let success = Rc::new(Mutex::new(Some(sender)));
    let error = Rc::clone(&success);
    let success_callback = self::closure_once(move || {
        if let Some(success) = success.lock().ok().and_then(|mut success| success.take()) {
            if let Err(err) = success.send(Ok(())) {
                error!("Cannot send 'image loaded messsage': {:#?}", err);
            }
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = self::closure_once(move |err| {
        if let Some(error) = error.lock().ok().and_then(|mut error| error.take()) {
            if let Err(err) = error.send(Err(anyhow!("Error when loading image: {:#?}", err))) {
                error!("Cannot send 'image error message': {:#?}", err);
            }
        }
    });
    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(uri);
    receiver.await??;
    Ok(image)
}
