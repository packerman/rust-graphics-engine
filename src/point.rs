use web_sys::WebGl2RenderingContext;

use crate::core::{application::Application, color, gl::set_clear_color};

const VERTEX_SHADER_CODE: &str = "
void main()
{
    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
}
";

const FRAGMENT_SHADER_CODE: &str = "
out vec4 fragColor;
void main()
{
    fragColor = vec4(1.0, 1.0, 0.0, 1.0);
}
";

pub struct PointApp;

impl PointApp {
    pub fn create(context: &WebGl2RenderingContext) -> Box<dyn Application> {
        set_clear_color(context, &color::black());
        Box::new(PointApp)
    }
}

impl Application for PointApp {
    fn update(&mut self) {}
    fn render(&self, context: &WebGl2RenderingContext) {
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}
