use sdl2::event::Event;

extern crate gl;
extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let load_function = |s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void;
    let _gl = gl::load_with(load_function);

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::AppTerminating { timestamp } => {}
                Event::AppLowMemory { timestamp } => {}
                Event::AppWillEnterBackground { timestamp } => {}
                Event::AppDidEnterBackground { timestamp } => {}
                Event::AppWillEnterForeground { timestamp } => {}
                Event::AppDidEnterForeground { timestamp } => {}
                Event::Window { timestamp, window_id, win_event } => {}
                Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => {}
                Event::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat } => {}
                Event::TextEditing { timestamp, window_id, text, start, length } => {}
                Event::TextInput { timestamp, window_id, text } => {print!("text! -> {}", text)}
                Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {}
                Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {}
                Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {}
                Event::MouseWheel { timestamp, window_id, which, x, y, direction } => {}
                Event::JoyAxisMotion { timestamp, which, axis_idx, value } => {}
                Event::JoyBallMotion { timestamp, which, ball_idx, xrel, yrel } => {}
                Event::JoyHatMotion { timestamp, which, hat_idx, state } => {}
                Event::JoyButtonDown { timestamp, which, button_idx } => {}
                Event::JoyButtonUp { timestamp, which, button_idx } => {}
                Event::JoyDeviceAdded { timestamp, which } => {}
                Event::JoyDeviceRemoved { timestamp, which } => {}
                Event::ControllerAxisMotion { timestamp, which, axis, value } => {}
                Event::ControllerButtonDown { timestamp, which, button } => {}
                Event::ControllerButtonUp { timestamp, which, button } => {}
                Event::ControllerDeviceAdded { timestamp, which } => {}
                Event::ControllerDeviceRemoved { timestamp, which } => {}
                Event::ControllerDeviceRemapped { timestamp, which } => {}
                Event::FingerDown { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {}
                Event::FingerUp { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {}
                Event::FingerMotion { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => {}
                Event::DollarGesture { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => {}
                Event::DollarRecord { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => {}
                Event::MultiGesture { timestamp, touch_id, d_theta, d_dist, x, y, num_fingers } => {}
                Event::ClipboardUpdate { timestamp } => {}
                Event::DropFile { timestamp, window_id, filename } => {}
                Event::DropText { timestamp, window_id, filename } => {}
                Event::DropBegin { timestamp, window_id } => {}
                Event::DropComplete { timestamp, window_id } => {}
                Event::AudioDeviceAdded { timestamp, which, iscapture } => {}
                Event::AudioDeviceRemoved { timestamp, which, iscapture } => {}
                Event::RenderTargetsReset { timestamp } => {}
                Event::RenderDeviceReset { timestamp } => {}
                Event::User { timestamp, window_id, type_, code, data1, data2 } => {}
                Event::Unknown { timestamp, type_ } => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}
