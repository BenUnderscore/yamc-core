use gl::types::*;
use glutin;

pub struct RenderState {
    ctx: glutin::RawContext<glutin::PossiblyCurrent>,
    vertex_shader: Option<GLuint>,
    fragment_shader: Option<GLuint>,
    shader_program: Option<GLuint>,
}

impl RenderState {
    pub fn init(ctx: glutin::RawContext<glutin::NotCurrent>) -> RenderState {
        let current_ctx = unsafe { ctx.make_current().unwrap() };

        gl::load_with(|s| current_ctx.get_proc_address(s));

        RenderState {
            ctx: current_ctx,
            vertex_shader: None,
            fragment_shader: None,
            shader_program: None,
        }
    }
}
