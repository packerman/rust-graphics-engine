use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Ok, Result};
use wasm_bindgen::prelude::Closure;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::core::gl;

use super::{
    input::{KeyState, KeyboardInput},
    web,
};

pub trait Application {
    fn update(&mut self, key_state: &KeyState);
    fn render(&self, context: &WebGl2RenderingContext);
}

type Creator<T> = dyn Fn(&WebGl2RenderingContext) -> Result<T>;
pub type ApplicationCreator = Creator<Box<dyn Application>>;

pub struct Loop {
    previous_time: f64,
    lag: f64,
}

impl Loop {
    const FRAMES_PER_SECOND: i32 = 60;
    const MS_PER_UPDATE: f64 = 1000.0 / (Self::FRAMES_PER_SECOND as f64);

    pub fn run_with_box(
        canvas: &HtmlCanvasElement,
        creator: Box<ApplicationCreator>,
    ) -> Result<()> {
        Self::run(canvas, &creator)
    }

    pub fn run(canvas: &HtmlCanvasElement, creator: &ApplicationCreator) -> Result<()> {
        let context = web::get_webgl2_context(canvas)?;
        log_gl_strings(&context)?;
        let mut app = creator(&context)?;
        let mut state = Loop {
            previous_time: web::now()?,
            lag: 0.0,
        };
        let mut key_state = KeyState::new();
        let mut keyboard_input = KeyboardInput::prepare(canvas);
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |current_time| {
            keyboard_input.process(&mut key_state);
            let elapsed = current_time - state.previous_time;
            state.previous_time = current_time;
            state.lag += elapsed;
            while state.lag >= Self::MS_PER_UPDATE {
                app.update(&key_state);
                state.lag -= Self::MS_PER_UPDATE;
            }
            app.render(&context);
            web::request_animation_frame(
                f.borrow()
                    .as_ref()
                    .expect("Empty reference to the `requestAnimationFrame` callback"),
            )
            .expect("Cannot run `requestAnimationFrame`");
        }) as Box<dyn FnMut(f64)>));
        web::request_animation_frame(
            g.borrow().as_ref().ok_or_else(|| {
                anyhow!("Empty reference to the `requestAnimationFrame` callback")
            })?,
        )?;
        Ok(())
    }
}

fn log_gl_strings(context: &WebGl2RenderingContext) -> Result<()> {
    log!(
        "GL vendor = {}",
        gl::get_string_parameter(context, WebGl2RenderingContext::VENDOR)?
    );
    log!(
        "GL renderer = {}",
        gl::get_string_parameter(context, WebGl2RenderingContext::RENDERER)?
    );
    log!(
        "GL version = {}",
        gl::get_string_parameter(context, WebGl2RenderingContext::VERSION)?
    );
    log!(
        "GLSL version = {}",
        gl::get_string_parameter(context, WebGl2RenderingContext::SHADING_LANGUAGE_VERSION)?
    );
    Ok(())
}
