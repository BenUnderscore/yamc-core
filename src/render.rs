//Uses
use crate::res::ResourceSystem;

use gl::types::*;
use glutin;

pub struct RenderState {
    ctx: glutin::RawContext<glutin::PossiblyCurrent>,
    vertex_shader: Option<GLuint>,
    fragment_shader: Option<GLuint>,
    shader_program: Option<GLuint>,
}

impl RenderState {
    pub fn init(ctx: glutin::RawContext<glutin::NotCurrent>, resources: &mut ResourceSystem) -> RenderState {
        let current_ctx = unsafe { ctx.make_current().unwrap() };

        gl::load_with(|s| current_ctx.get_proc_address(s));

        RenderState {
            ctx: current_ctx,
            vertex_shader: None,
            fragment_shader: None,
            shader_program: None,
        }
    }

    pub fn render(&self) {
        if !self.ctx.is_current() {
            panic!("Render called on non-render thread!");
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.ctx.swap_buffers().unwrap();
    }
}
