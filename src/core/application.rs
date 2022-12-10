use std::rc::Rc;

use anyhow::{anyhow, Result};

use async_trait::async_trait;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use super::{
    input::{KeyState, KeyboardInput},
    web,
};

#[async_trait(?Send)]
pub trait AsyncCreator {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>>;
}

pub trait Application {
    fn update(&mut self, key_state: &KeyState);
    fn render(&self, context: &WebGl2RenderingContext);
}

pub fn spawn<C: AsyncCreator + Application + 'static>() {
    web::spawn_local(async {
        let canvas = web::get_canvas_by_id("canvas").expect("Cannot find canvas");
        Loop::run::<C>(&canvas)
            .await
            .expect("Cannot run application");
    });
}

pub struct Loop {
    previous_time: f64,
    lag: f64,
}

impl Loop {
    const FRAMES_PER_SECOND: i32 = 60;
    const MS_PER_UPDATE: f64 = 1000.0 / (Self::FRAMES_PER_SECOND as f64);
    pub const SECS_PER_UPDATE: f64 = 1.0 / (Self::FRAMES_PER_SECOND as f64);

    pub async fn run<C: AsyncCreator + Application + 'static>(
        canvas: &HtmlCanvasElement,
    ) -> Result<()> {
        let context = Rc::new(web::get_webgl2_context(canvas)?);
        auto_resize_canvas(Rc::clone(&context))?;
        let mut app = C::create(&context).await?;
        let mut state = Loop {
            previous_time: web::now()?,
            lag: 0.0,
        };
        let mut key_state = KeyState::new();
        let mut keyboard_input = KeyboardInput::prepare(canvas);
        web::request_animation_loop(move |current_time| {
            keyboard_input.process(&mut key_state);
            let elapsed = current_time - state.previous_time;
            state.previous_time = current_time;
            state.lag += elapsed;
            while state.lag >= Self::MS_PER_UPDATE {
                app.update(&key_state);
                state.lag -= Self::MS_PER_UPDATE;
            }
            app.render(&context);
        })?;
        Ok(())
    }
}

pub fn auto_resize_canvas(context: Rc<WebGl2RenderingContext>) -> Result<()> {
    fn expand_full_screen(context: Rc<WebGl2RenderingContext>) {
        if let Ok(window) = web::window() {
            if let Ok((width, height)) = web::window_inner_size(&window) {
                if let Ok(canvas) = web::get_canvas(&context) {
                    debug!("Canvas resize: {}x{}", width, height);
                    web::set_canvas_size(&canvas, (width as u32, height as u32))
                }
            }
        }
    }
    expand_full_screen(Rc::clone(&context));
    let closure = Closure::wrap(
        Box::new(move || expand_full_screen(Rc::clone(&context))) as Box<dyn FnMut()>
    );
    web::window()?
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .map_err(|err| anyhow!("Cannot register windows resize callback {:#?}", err))?;
    closure.forget();
    Ok(())
}
