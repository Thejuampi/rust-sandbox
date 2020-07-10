extern crate sdl2;
extern crate gl;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .unwrap();


    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }
    }

}
