extern crate sdl2;

fn main() {
    println!("booting...");
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap();

    // render loop
    let mut event_pump = sdl.event_pump().unwrap();
    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'game,
                _ => {}
            }
        }
    }
}
