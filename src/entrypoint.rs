use crate::core::Game;
use raylib::prelude::*;

pub fn entrypoint() {
    set_trace_log(TraceLogType::LOG_TRACE);

    let (mut rl, thread) = raylib::init()
        .msaa_4x()
        .vsync()
        .size(640, 480)
        .title("The Slopes")
        .build();

    trace_log(TraceLogType::LOG_DEBUG, "initializing threadpool");
    let num_threads = num_cpus::get();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();
    trace_log(TraceLogType::LOG_DEBUG, "initialized threadpool");

    let mut game = Game::default();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::SKYBLUE);
        d.draw_fps(0, 0);
    }
}
