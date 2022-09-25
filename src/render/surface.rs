//! Handles the top-level surface

pub struct RenderSurface {
    //Surface constants
    surface: wgpu::Surface,
    surface_format: wgpu::TextureFormat,
    size_x: u32,
    size_y: u32,
}

impl RenderSurface {
    pub fn init(
        device: &wgpu::Device,
        surface: wgpu::Surface,
        size_x: u32,
        size_y: u32,
        format: wgpu::TextureFormat,
    ) -> RenderSurface {
        let ctx = RenderSurface {
            surface,
            surface_format: format,
            size_x,
            size_y,
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

    pub fn get_surface_texture(&self) -> wgpu::SurfaceTexture {
        self.surface.get_current_texture().unwrap()
    }
}
