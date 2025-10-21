use std::{process, thread};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use eframe::NativeOptions;
use egui::{Context, ThemePreference};
use egui_wgpu::{Renderer, RendererOptions};
use rfd::MessageLevel;
use wgpu::{Adapter, Backends, CreateSurfaceError, Device, Instance, InstanceDescriptor, PollError, PollStatus, PowerPreference, Queue, RequestAdapterError, RequestDeviceError, Surface, SurfaceConfiguration};
use wgpu::wgt::{DeviceDescriptor, PollType, RequestAdapterOptions};
use wgpu::wgt::PollType::Wait;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::error::{EventLoopError, OsError};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Theme, Window, WindowAttributes, WindowId};
use gammaeditor::app::LegacyApp;
use gammaeditor::logger::Logger;
use gammaeditor::utils::fatal_error_dialog;

fn main() {
    env_logger::init();
    Logger::init().unwrap();

    let event_loop = match EventLoop::new() {
        Ok(l) => {l}
        Err(string) => {panic!("{}", string)}
    };

    let native_options: NativeOptions = NativeOptions::default();
    let app_name: &str = "GammaEditor";

    let mut app = App::new();
    let _ = match event_loop.run_app(&mut app) {
        Ok(app) => {app}
        Err(error) => {
            rfd::MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title("Fatal Error")
                .set_description(error.to_string())
                .show();

            panic!()
        }
    };
}

struct App {
    state: Option<State>,
    egui_ctx: Context,
    egui_renderer: Option<Renderer>,
    egui_state: Option<egui_winit::State>,
    needs_configure: bool
}


impl App {
    pub fn new() -> Self {
        Self {
            state: None,
            egui_ctx: Context::default(),
            egui_renderer: None,
            egui_state: None,
            needs_configure: true
        }
    }
}

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,
    egui_state: EguiState
}

pub struct EguiState {
    context: Context,
    renderer: Renderer,
    state: egui_winit::State,
}

impl State {
    fn window(event_loop: &ActiveEventLoop) -> Window {
        let attributes: WindowAttributes = Window::default_attributes()
            .with_title("GammaEditor")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let window = match event_loop.create_window(attributes) {
            Ok(window) => {
                Logger::info("Window created");
                window
            }
            Err(error) => {
                rfd::MessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_title("Fatal error")
                    .set_description(error.to_string())
                    .show();

                panic!()
            }
        };

        window
    }

    fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
                ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let opts: RequestAdapterOptions<&Surface> = wgpu::RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        };

        let adapter: Adapter = match pollster::block_on(instance.request_adapter(&opts)) {
            Ok(a) => {
                let info = a.clone().get_info();
                Logger::info("Created adapter with GPU:");
                Logger::info(format!("Name: {}", info.name));
                Logger::info(format!("Device: {}", info.device));
                Logger::info(format!("DeviceType: {:?}", info.device_type));
                Logger::info(format!("Driver: {}", info.driver));
                Logger::info(format!("DriverInfo: {}", info.driver_info));
                Logger::info(format!("Backend: {}", info.backend));
                Logger::info(format!("Vendor: {}", info.vendor));

                a
            }
            Err(error) => {
                fatal_error_dialog(error.to_string()).show();
                panic!("{}", "No GPU Found")
            }
        };

        let req: Result<(Device, Queue), RequestDeviceError> = pollster::block_on(adapter.request_device(&DeviceDescriptor::default()));
        let (device, queue): (Device, Queue) = match req {
            Ok((dv,q)) => { (dv,q)}
            Err(error) => {
                fatal_error_dialog(error.to_string()).show();
                panic!()
            }
        };

        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("surface config");

        let egui_state = Self::get_egui_state(&window, &device, config.clone());

        Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            egui_state: egui_state
        }
    }

    pub fn get_egui_state(window: &Window, device: &Device, config: SurfaceConfiguration) -> EguiState {
        let egui_ctx: Context = Context::default();
        let egui_renderer: Renderer = Renderer::new(
            device,
            config.format,
            RendererOptions::default(), 
        );
        let egui_state: egui_winit::State = egui_winit::State::new(
            Context::default(),
            egui::ViewportId::ROOT,
            window,
            None,
            Some(Theme::Dark),
            None,
        );

        EguiState {
            context: egui_ctx,
            renderer: egui_renderer,
            state: egui_state,
        }
    }

    pub fn handle_redraw(&mut self) {
        let window = &self.window;
        if let Some(min) = window.is_minimized() {
            if min == true {
                Logger::info("Will not handle_redraw as the window is minimized");
                return;
            }
        }

        let window = self.window.as_ref();

        let self.self.egui_state.renderer.begin_frame()


    }

    pub fn handle_resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            Logger::info(format!("Surface configured for res: {}x{}", new_size.width, new_size.height));
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window: Arc<Window> = Arc::new(State::window(&event_loop));
        let state: State = State::new(window.clone());

        let (device, config) = (&state.device, &state.config);
        let egui_ctx = Context::default();
        let egui_renderer = Renderer::new(device, config.format, RendererOptions::default());
        let egui_state = egui_winit::State::new(
            egui_ctx,
            egui::ViewportId::ROOT,
            &window,
            None,
            Some(Theme::Dark),
            None,
        );

        self.state = Some(state);
        self.egui_renderer = Some(egui_renderer);
        self.egui_state = Some(egui_state);

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if let Some(state) = &self.state {
            let window = &state.window;
            if window.id() == window_id {
                let _ = match event {
                    WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::RedrawRequested => {
                        if self.needs_configure {
                            let (surface, device, config) = (&state.surface, &state.device, &state.config);
                            // pollster::block_on(device.poll())
                            Logger::info("Waiting for GPU to go idle...");
                            let poll = device.poll(PollType::Wait {
                                submission_index: None,
                                timeout: None,
                            });
                            match poll {
                                Ok(_) => {}
                                Err(error) => {
                                    Logger::info(format!("Fatal error: {}", error));
                                }
                            }
                            thread::sleep(Duration::from_millis(1000));
                            Logger::info("Configuring surface...");

                            surface.configure(&device, &config);

                            self.needs_configure = false;
                            Logger::info("Surface configured!");

                        }
                    },
                    WindowEvent::Resized(new_size) => {
                        match self.state.as_mut() {
                            None => {}
                            Some(state) => {
                                state.handle_resize(new_size);
                            }
                        }
                    }
                    _ => {}
                };
            }
        }
    }
}

fn run_generator(args: Vec<String>) -> Result<String, String> {
    println!("Starting generator");

    let output = process::Command::new("bin/generator.exe")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run generator.exe: {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}