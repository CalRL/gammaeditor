
use egui::{Context, ViewportId};
use egui_wgpu::{Renderer, RendererOptions};
use wgpu::{Device, TextureFormat};
use winit::window::Window;

pub struct EguiRenderer {
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
    frame_started: bool,
}

impl EguiRenderer {
    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        msaa_samples: u32,
        window: &Window
    ) -> Self {
        let egui_context = Context::default();
        let egui_state = egui_winit::State::new(
            egui_context,
            ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(2 * 1024)
        );
        let mut renderer_options = RendererOptions::default();
        renderer_options.msaa_samples = msaa_samples;

        let egui_renderer = Renderer::new(
            device,
            output_color_format,
            renderer_options,
        );

        Self {
            state: egui_state,
            renderer: egui_renderer,
            frame_started: false,
        }
    }
}