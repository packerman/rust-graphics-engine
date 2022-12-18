use std::rc::Rc;

use anyhow::Result;

use async_trait::async_trait;

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
            self::resize_canvas(&context).expect("Error while resizing canvas");
            app.render(&context);
        })?;
        Ok(())
    }
}

fn resize_canvas(context: &WebGl2RenderingContext) -> Result<()> {
    let canvas = web::get_canvas(context)?;
    let width = canvas.client_width() as u32;
    let height = canvas.client_height() as u32;
    if canvas.width() != width || canvas.height() != height {
        canvas.set_width(width);
        canvas.set_height(height);
    }
    Ok(())
}
