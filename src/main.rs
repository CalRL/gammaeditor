use std::process;
use eframe::NativeOptions;
use egui::{Context, ThemePreference};
use egui_wgpu::{Renderer, RendererOptions};
use egui_winit::State;
use rfd::MessageLevel;
use wgpu::{Adapter, CreateSurfaceError, Device, Instance, PowerPreference, Queue, RequestAdapterError, RequestDeviceError, Surface, SurfaceConfiguration};
use wgpu::wgt::{DeviceDescriptor, PollType, RequestAdapterOptions};
use wgpu::wgt::PollType::Wait;
use winit::application::ApplicationHandler;
use winit::error::{EventLoopError, OsError};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Theme, Window, WindowAttributes, WindowId};
use gammaeditor::app::LegacyApp;
use gammaeditor::logger::Logger;
use gammaeditor::utils::fatal_error_dialog;

fn main() {
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
    window: Option<Window>,
    surface: Option<Surface<'static>>,
    instance: Option<Instance>,
    device: Option<Device>,
    config: Option<SurfaceConfiguration>,
    egui_ctx: Context,
    egui_renderer: Option<Renderer>,
    egui_state: Option<State>,
    needs_configure: bool
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            surface: None,
            instance: None,
            device: None,
            config: None,
            egui_ctx: Context::default(),
            egui_renderer: None,
            egui_state: None,
            needs_configure: true
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
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



        let instance: Instance = Instance::default();
        let surface: Surface =
            unsafe {
                std::mem::transmute::<wgpu::Surface<'_>, wgpu::Surface<'static>>(
                    instance.create_surface(&window).unwrap(),
                )
            };

        Logger::info("Surface created");

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
        let (device, _): (Device, Queue) = match req {
            Ok((dv,q)) => { (dv,q)}
            Err(error) => {
                fatal_error_dialog(error.to_string()).show();
                panic!()
            }
        };

        let size = &window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("surface config");

        let egui_ctx = Context::default();
        let egui_renderer = Renderer::new(&device, config.format, RendererOptions::default());
        let egui_state = egui_winit::State::new(
            egui_ctx,
            egui::ViewportId::ROOT,
            &window,
            None,
            Some(Theme::Dark),
            None,
        );

        self.window = Some(window);
        self.surface = Some(surface);
        self.instance = Some(instance);
        self.device = Some(device);
        self.config = Some(config);
        self.egui_renderer = Some(egui_renderer);
        self.egui_state = Some(egui_state);

    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if let Some(window) = &self.window {
            if window.id() == window_id {
                let _ = match event {
                    WindowEvent::CloseRequested => event_loop.exit(),
                    WindowEvent::RedrawRequested => {
                        if self.needs_configure {
                            if let (Some(surface), Some(device), Some(config)) =
                                (&self.surface, &self.device, &self.config)
                            {
                                // pollster::block_on(device.poll())
                                Logger::info("Configuring surface...");
                                surface.configure(device, config);
                                self.needs_configure = false;
                                Logger::info("Surface configured!");
                            }
                        }
                    },
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