// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - main.rs

mod vector;
mod matrix;
mod ray;
mod camera;
mod raytracer;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};
use egui_wgpu::wgpu;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct LumaApplication
{
    name: String,
    raytracer: raytracer::Raytracer,
    texture: Option<egui::TextureHandle>,
    initialized: bool,
}

impl LumaApplication
{
    pub fn new() -> Self
    {
        return Self
        {
            name: "Luma Pathtracer".to_owned(),
            raytracer: raytracer::Raytracer::new(1000, 1000),
            texture: None,
            initialized: false,
        }
    }
    pub fn egui_ui(&mut self, ctx: &egui::Context)
    {
        egui::CentralPanel::default().show(ctx, |_ui|
        {
        });
    }
}

fn create_window(event_loop: &winit::event_loop::ActiveEventLoop) -> winit::window::Window
{
    let window_attributes = winit::window::WindowAttributes {
        title: "Luma Pathtracer".to_string(),
        inner_size: Some(winit::dpi::LogicalSize::new(1000.0, 1000.0).into()),
        ..Default::default()
    };
    event_loop.create_window(window_attributes).expect("No se pudo crear la ventana")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() 
{
    let event_loop = EventLoop::new();
    event_loop.unwrap().run_app(|event_loop| {
        let window = create_window(event_loop);

        // ...el resto de tu inicialización y bucle de eventos...
        // Mueve aquí el código que estaba dentro de event_loop.run(...)
    });

    #[cfg(not(target_arch = "wasm32"))]
    let instance =
    {
        let backend = wgpu::Backends::all();
        let compiler = wgpu::Dx12Compiler::default();
        wgpu::Instance::new(&wgpu::InstanceDescriptor
        {
            backends: backend,
            flags: wgpu::InstanceFlags::default(),
            backend_options: wgpu::BackendOptions
            {
                gl: wgpu::GlBackendOptions::default(),
                dx12: wgpu::Dx12BackendOptions::default(),
                noop: wgpu::NoopBackendOptions::default(),
            },
        })
    };

    #[cfg(target_arch = "wasm32")]
    let instance =
    {
        let backend = wgpu::Backends::all();
        wgpu::Instance::new(wgpu::InstanceDescriptor
        {
            backends: backend,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc, // Valor por defecto, ignorado en web
        })
    };

    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions 
    {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.unwrap();

    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor
    {
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        label: None,
        memory_hints: wgpu::MemoryHints::default(),
        trace: wgpu::Trace::Off,
    }).await.unwrap();

    let size = window.inner_size();
    let surface_format = surface.get_capabilities(&adapter).formats[0];
    let mut config = wgpu::SurfaceConfiguration
    {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        desired_maximum_frame_latency: 2,
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let mut egui_ctx = egui::Context::default();
    let mut egui_state = EguiWinitState::new(&window);
    let mut egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1, false);

    let mut app = LumaApplication::new();

    event_loop.run(move |event, _, control_flow|
    {
        *control_flow = ControlFlow::Poll;

        match event
        {
            Event::WindowEvent { event, .. } =>
            {
                if egui_state.on_event(&egui_ctx, &event).consumed
                {
                    return;
                }
                match event
                {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Wait,
                    WindowEvent::Resized(new_size) =>
                    {
                        config.width = new_size.width;
                        config.height = new_size.height;
                        surface.configure(&device, &config);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) =>
            {
                let raw_input = egui_state.take_egui_input(&window);
                egui_ctx.begin_pass(raw_input);

                app.egui_ui(&egui_ctx);

                let full_output = egui_ctx.end_pass();
                let paint_jobs = egui_ctx.tessellate(full_output.shapes);

                let output_frame = match surface.get_current_texture()
                {
                    Ok(frame) => frame,
                    Err(_) =>
                    {
                        surface.configure(&device, &config);
                        surface.get_current_texture().unwrap()
                    }
                };
                let view = output_frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("encoder") });

                let screen_descriptor = ScreenDescriptor
                {
                    size_in_pixels: [config.width, config.height],
                    pixels_per_point: egui_ctx.pixels_per_point(),
                };

                egui_renderer.update_buffers(&device, &queue, &mut encoder, &paint_jobs, &screen_descriptor);
                egui_renderer.render(&mut encoder, &view, &paint_jobs, &screen_descriptor, None);

                queue.submit(Some(encoder.finish()));
                output_frame.present();
            }
            Event::MainEventsCleared =>
            {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main()
{
    pollster::block_on(run());
}

// use eframe::egui;


// impl LumaApplication
// {
//     
// }

// struct GeoApplication
// {
//     name: String,
//     rasterizer: rasterizer::Rasterizer,
//     initialized: bool,
// }

// impl eframe::App for LumaApplication
// {
//     fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4]
//     {
//         return [0.0, 0.0, 0.0, 1.0];    
//     }

//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
//     {
//         if !self.initialized
//         {
//             egui_extras::install_image_loaders(ctx);
//             self.initialized = true;
//         }

//         self.raytracer.update(ctx);
//         self.raytracer.render(2);

//         // obtain a texture handle from the framebuffer bitmap
//         let bitmap = self.raytracer.bitmap();
//         let size = self.raytracer.size();
//         let image = egui::ColorImage::from_rgba_unmultiplied(size, &bitmap);
//         self.texture = Some(ctx.load_texture("image", image, egui::TextureOptions::LINEAR));

//         egui::CentralPanel::default()
//             .frame(egui::Frame::NONE)
//             .show(ctx, |ui|
//         {
//             if let Some(texture) = &self.texture
//             {
//                 let size = texture.size_vec2();
//                 ui.add(egui::Image::new((texture.id(), size)));
//             }
//         });

//         let layer_id = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("frametime_layer"));
//         egui::Area::new(layer_id.id)
//             .fixed_pos(egui::pos2(10.0, 10.0)) // Position the label at the top-left corner
//             .default_size(egui::Vec2::new(500.0, 100.0)) // Set a default size for the area
//             .show(ctx, |ui|
//         {
//             let visuals = ui.visuals_mut();
//             visuals.override_text_color = Some(egui::Color32::BLACK);

//             ui.vertical(|ui|
//             {
//                 ui.label("Luma Pathtracer");
//                 ui.label(format!("Frametime: {}", self.raytracer.frametime()));

//                 let position = self.raytracer.position();
//                 ui.label(format!("Position: ({}, {}, {})", position.x(), position.y(), position.z()));

//                 let rotation = self.raytracer.rotation();
//                 ui.label(format!("Rotation: ({}, {}, {})", rotation.x(), rotation.y(), rotation.z()));
//             });
//         });
//     }
// }

// impl eframe::App for GeoApplication
// {
//     fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4]
//     {
//         return [0.0, 0.0, 0.0, 1.0];    
//     }

//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
//     {
    
//     }
// }


// fn run_raytracer()
// {
//     let application = LumaApplication::new();

//     let options = eframe::NativeOptions
//     {
//         viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
//         ..Default::default()
//     };

//     let _ = eframe::run_native
//     (
//         "Luma Pathtracer",
//         options,
//         Box::new(|_cc| Ok(Box::new(application))),
//     );
// }

// fn run_rasterizer()
// {

// }

// fn main()
// {
//     let command_line = std::env::args().collect::<Vec<String>>();
//     for arg in command_line.iter()
//     {
//         if arg == "--raytrace"
//         {
//             run_raytracer();
//         }
//         else if arg == "--rasterize"
//         {
//             run_rasterizer();
//         }
//         else
//         {
//             println!("Usage: luma [options]");
//             return;
//         }
//     }
// }
