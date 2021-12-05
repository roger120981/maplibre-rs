pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

pub const DEPTH_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24PlusStencil8;

impl Texture {

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        _label: &str,
        sample_count: u32,
    ) -> Self {

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_TEXTURE_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        });
        let view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture: depth_texture,
            view,
        }
    }
}