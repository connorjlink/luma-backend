// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - main.rs

mod vector;
mod matrix;
mod ray;
mod camera;
mod renderer;

use eframe::egui;

struct LumaApplication
{
    name: String,
    renderer: renderer::Renderer,
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
            renderer: renderer::Renderer::new(1000, 1000),
            texture: None,
            initialized: false,
        }
    }
}

impl eframe::App for LumaApplication
{
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4]
    {
        return [0.0, 0.0, 0.0, 1.0];    
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        if !self.initialized
        {
            egui_extras::install_image_loaders(ctx);
            self.initialized = true;
        }

        self.renderer.update(ctx);
        self.renderer.render(2);
    

        // obtain a texture handle from the framebuffer bitmap
        let bitmap = self.renderer.bitmap();
        let size = self.renderer.size();
        let image = egui::ColorImage::from_rgba_unmultiplied(size, &bitmap);
        self.texture = Some(ctx.load_texture("image", image, egui::TextureOptions::LINEAR));

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui|
        {
            if let Some(texture) = &self.texture
            {
                let size = texture.size_vec2();
                ui.add(egui::Image::new((texture.id(), size)));
            }
        });

        let layer_id = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("frametime_layer"));
        egui::Area::new(layer_id.id)
            .fixed_pos(egui::pos2(10.0, 10.0)) // Position the label at the top-left corner
            .default_size(egui::Vec2::new(500.0, 100.0)) // Set a default size for the area
            .show(ctx, |ui|
        {
            let visuals = ui.visuals_mut();
            visuals.override_text_color = Some(egui::Color32::BLACK);

            ui.vertical(|ui|
            {
                ui.label("Luma Pathtracer");
                ui.label(format!("Frametime: {}", self.renderer.frametime()));

                let position = self.renderer.position();
                ui.label(format!("Position: ({}, {}, {})", position.x(), position.y(), position.z()));

                let rotation = self.renderer.rotation();
                ui.label(format!("Rotation: ({}, {}, {})", rotation.x(), rotation.y(), rotation.z()));
            });
        });
    }
}

fn main()
{
    let application = LumaApplication::new();

    let options = eframe::NativeOptions
    {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 1000.0]),
        ..Default::default()
    };

    let _ = eframe::run_native
    (
        "Luma Pathtracer",
        options,
        Box::new(|_cc| Ok(Box::new(application))),
    );
}
