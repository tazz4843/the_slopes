use raylib::prelude::*;

pub fn entrypoint() {
    let (mut rl, thread) = raylib::init().msaa_4x().size(640, 480).title("The Slopes").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::SKYBLUE);
        d.draw_fps(0, 0);
    }
}
