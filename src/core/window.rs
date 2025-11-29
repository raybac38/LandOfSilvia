use crate::system::render_system::{self, RenderSystem};



pub struct Window {
    raylib_handler : raylib::RaylibHandle,
    thread : raylib::RaylibThread,
    render_system : RenderSystem,
    is_open : bool
}

impl Window {
    pub fn new(size_x : i32, size_y : i32, title : &str, render_system : RenderSystem) -> Self {
        let (rl, thread) = raylib::init()
        .size(size_x, size_y)
        .title(title)
        .build();

        Window { raylib_handler: rl, thread, render_system, is_open : true }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn render_frame(& mut self) {
        let d = self.raylib_handler.begin_drawing(&self.thread);
        self.render_system.render_frame(d);

        if self.raylib_handler.window_should_close() {
            self.is_open = false;
        }
    }
   

}