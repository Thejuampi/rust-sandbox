use sdl2::event::Event;

pub fn init_and_loop() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
      .window("Game", 900, 700)
      .opengl()
      .resizable()
      .build()
      .unwrap();
  
    let _gl_context = window.gl_create_context()?;
    let load_function = |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void;
    let _gl = gl::load_with(load_function);
  
    unsafe {
      gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
  
    let mut event_pump = sdl_context.event_pump()?;
    'main: loop {
      for event in event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => break 'main,
          _ => {}
        }
      }
  
      unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
      }
  
      window.gl_swap_window();
    }
  
    return Ok(());
  }