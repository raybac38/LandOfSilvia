use raylib::{
    camera::Camera3D, color::Color, ffi::Model, prelude::{RaylibDraw, RaylibDraw3D, RaylibDrawHandle, RaylibMode3DExt}
};

struct Render {
    models: Vec<Model>,
    camera: Camera3D
}

impl Render {
    fn new() -> Render {
        return Render { models: Vec::new(), camera: raylib::camera::Camera3D }
    }
    fn update(&self, mut d: RaylibDrawHandle) {
        d.clear_background(Color::WHITE);
        d.draw_text(&d.get_fps().to_string(), 0, 0, 30, Color::BLACK);

        /*let mut d3d = d.begin_mode3D(self.camera);

        for i in &self.models {
        }*/
    }
}
