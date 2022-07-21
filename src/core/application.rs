use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{anyhow, Ok, Result};
use wasm_bindgen::prelude::Closure;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::core::gl::get_string_parameter;

use super::web::{self, get_webgl2_context, request_animation_frame};

pub trait Application {
    fn update(&mut self);
    fn render(&self, context: &WebGl2RenderingContext);
}

pub type AppCreator = dyn FnOnce(&WebGl2RenderingContext) -> Box<dyn Application>;

pub struct Loop {
    previous_time: f64,
    lag: f64,
}

impl Loop {
    const FRAMES_PER_SECOND: i32 = 60;
    const MS_PER_UPDATE: f64 = 1000.0 / (Self::FRAMES_PER_SECOND as f64);

    pub fn run(canvas: &HtmlCanvasElement, creator: Box<AppCreator>) -> Result<()> {
        let context = get_webgl2_context(&canvas)?;
        log_gl_strings(&context)?;
        let mut app = creator(&context);
        let mut state = Loop {
            previous_time: web::now()?,
            lag: 0.0,
        };
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move |current_time| {
            let elapsed = current_time - state.previous_time;
            state.previous_time = current_time;
            state.lag += elapsed;
            while state.lag >= Self::MS_PER_UPDATE {
                app.update();
                state.lag -= Self::MS_PER_UPDATE;
            }
            app.render(&context);
            request_animation_frame(
                f.borrow()
                    .as_ref()
                    .expect("Empty reference to the `requestAnimationFrame` callback"),
            )
            .expect("Cannot run `requestAnimationFrame`");
        }) as Box<dyn FnMut(f64)>));
        request_animation_frame(
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
        get_string_parameter(context, WebGl2RenderingContext::VENDOR)?
    );
    log!(
        "GL renderer = {}",
        get_string_parameter(context, WebGl2RenderingContext::RENDERER)?
    );
    log!(
        "GL version = {}",
        get_string_parameter(context, WebGl2RenderingContext::VERSION)?
    );
    log!(
        "GLSL version = {}",
        get_string_parameter(&context, WebGl2RenderingContext::SHADING_LANGUAGE_VERSION)?
    );
    Ok(())
}
