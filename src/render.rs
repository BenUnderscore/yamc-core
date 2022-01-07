//Uses
use crate::res::{LoadedResourceData, ResourceLoadType, ResourceSystem};
use gl::types::*;
use glutin;
use std::ptr;

pub struct Renderer {
    ctx: glutin::RawContext<glutin::PossiblyCurrent>,
    vertex_shader: GLuint,
    fragment_shader: GLuint,
    shader_program: GLuint,
    vbo: GLuint,
    vao: GLuint,
}

impl Renderer {
    pub fn init(
        ctx: glutin::RawContext<glutin::NotCurrent>,
        resources: &mut ResourceSystem,
    ) -> Renderer {
        let current_ctx = unsafe { ctx.make_current().unwrap() };

        gl::load_with(|s| current_ctx.get_proc_address(s));

        unsafe {
            let vs_source_res = resources
                .get_loaded_resource("shaders/default-vert.glsl", ResourceLoadType::PlainText)
                .unwrap();
            let vs_shader_obj = match &vs_source_res.data {
                LoadedResourceData::Text(source) => {
                    create_shader_with_source(&source, gl::VERTEX_SHADER)
                }
                _ => panic!("Shader resource is of the wrong type!"),
            };

            let fs_source_res = resources
                .get_loaded_resource("shaders/default-frag.glsl", ResourceLoadType::PlainText)
                .unwrap();
            let fs_shader_obj = match &fs_source_res.data {
                LoadedResourceData::Text(source) => {
                    create_shader_with_source(&source, gl::FRAGMENT_SHADER)
                }
                _ => panic!("Shader resource is of the wrong type!"),
            };

            let shader_program = compile_shaders_into_program(vs_shader_obj, fs_shader_obj);

            let vbo = {
                let mut vbo = 0;
                gl::GenBuffers(1, ptr::addr_of_mut!(vbo));
                vbo
            };

            let vertices: Vec<f32> = vec![
                -0.5, -0.5, 0.0,
                0.0, 0.5, 0.0,
                0.5, -0.5, 0.0
            ];

            gl::NamedBufferData(
                vbo,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW
            );

            let vao = {
                let mut vao = 0;
                gl::GenVertexArrays(1, ptr::addr_of_mut!(vao));
                vao
            };

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                false as GLboolean,
                0,
                0 as *const std::ffi::c_void
            );

            Renderer {
                ctx: current_ctx,
                vertex_shader: vs_shader_obj,
                fragment_shader: fs_shader_obj,
                shader_program,
                vbo,
                vao
            }
        }
    }

    pub fn render(&self) {
        if !self.ctx.is_current() {
            panic!("Render called on non-render thread!");
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        self.ctx.swap_buffers().unwrap();
    }
}

unsafe fn create_shader_with_source(source: &str, shader_type: GLenum) -> GLuint {
    let shader_obj = gl::CreateShader(shader_type);
    let src_ptr = source.as_ptr();
    let src_byte_count = source.bytes().count() as GLint;
    gl::ShaderSource(
        shader_obj,
        1,
        ptr::addr_of!(src_ptr) as *const *const i8,
        ptr::addr_of!(src_byte_count),
    );

    shader_obj
}

unsafe fn compile_shader_with_diagnostics(shader_obj: GLuint) {
    gl::CompileShader(shader_obj);
    let compile_status = {
        let mut status: GLint = 0;
        gl::GetShaderiv(shader_obj, gl::COMPILE_STATUS, ptr::addr_of_mut!(status));
        status != 0
    };

    let info_log = {
        let mut buf: Vec<u8> = vec![0; 1024];
        let mut returned_length: GLsizei = 0;
        gl::GetShaderInfoLog(
            shader_obj,
            buf.len() as GLsizei,
            ptr::addr_of_mut!(returned_length),
            buf.as_mut_ptr() as *mut i8,
        );
        String::from_utf8(buf[0..returned_length as usize].to_vec()).unwrap()
    };

    if info_log.trim().len() > 0 {
        println!("Shader info log: \n{}", info_log);
    }

    if !compile_status {
        panic!("Failed to compile shader!");
    }
}

unsafe fn compile_shaders_into_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program_obj = gl::CreateProgram();
    gl::AttachShader(program_obj, vs);
    compile_shader_with_diagnostics(vs);
    gl::AttachShader(program_obj, fs);
    compile_shader_with_diagnostics(fs);

    gl::LinkProgram(program_obj);
    let link_status = {
        let mut status: GLint = 0;
        gl::GetProgramiv(program_obj, gl::LINK_STATUS, ptr::addr_of_mut!(status));
        status != 0
    };

    if !link_status {
        panic!("Shader program failed to link!");
    }

    program_obj
}
