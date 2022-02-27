//! Contains functionality that isn't related to rasterizing geometry

struct RenderContext {
    //Surface constants
    surface: wgpu::Surface,
    surface_format: wgpu::TextureFormat,
    size_x: u32,
    size_y: u32,

    //Auxilliary buffers
    depth_buffer: Option<wgpu::Texture>,
}

/// The output object for all of the 3D rendering operations
struct GeometryBuffers {
    pub albedo: wgpu::TextureView,
    pub depth: Option<wgpu::TextureView>,
}

impl RenderContext {
    pub fn init(device: &wgpu::Device, surface: wgpu::Surface, size_x: u32, size_y: u32) -> RenderContext {
        let surface_format = wgpu::TextureFormat::Rgba8Unorm;

        let ctx = RenderContext {
            surface,
            surface_format,
            depth_buffer: None,
            size_x,
            size_y
        };

        ctx.configure_surface(device);

        ctx
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size_x: u32, new_size_y: u32) {
        self.size_x = new_size_x;
        self.size_y = new_size_y;
        self.configure_surface(device);
    }

    fn configure_surface(&self, device: &wgpu::Device) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: self.size_x,
            height: self.size_y,
            present_mode: wgpu::PresentMode::Fifo,
        };
        self.surface.configure(device, &surface_config);
    }

    pub fn get_new_geometry_buffers(&self) -> GeometryBuffers {
        let surface_texture = self.surface.get_current_texture().unwrap();
    }

    pub fn present_frame(&self) {

    }
}