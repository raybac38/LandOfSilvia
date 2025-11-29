
mod render;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
     
    while !rl.window_should_close() {
        let d = rl.begin_drawing(&thread);
        render::update(d);
    }
}