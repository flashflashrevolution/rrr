use anyhow::Result;
use pixels::{
    raw_window_handle::HasRawWindowHandle, wgpu::Color, Pixels, PixelsBuilder, SurfaceTexture,
};

pub mod sprites;

pub struct Renderer {
    pub pixels: Pixels,
}

pub struct RendererBuilder<'win, W: HasRawWindowHandle> {
    color: Color,
    width: u32,
    height: u32,
    window: &'win W,
}

impl<'win, W: HasRawWindowHandle> RendererBuilder<'win, W> {
    pub fn new(width: u32, height: u32, window: &'win W) -> Self {
        let default_clear_color = pixels::wgpu::Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.0,
        };

        Self {
            width,
            height,
            window,
            color: default_clear_color,
        }
    }

    pub fn with_clear_color(&mut self, color: Color) {
        self.color = color;
    }

    pub async fn build(self) -> Result<Renderer> {
        let surface_texture = SurfaceTexture::new(self.width, self.height, self.window);
        let pixels = PixelsBuilder::new(self.width, self.height, surface_texture)
            .clear_color(self.color)
            .build_async()
            .await?;

        Ok(Renderer { pixels })
    }
}
