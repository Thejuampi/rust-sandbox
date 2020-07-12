extern crate gl;
extern crate sdl2;

mod opengl;

fn main() {
  match opengl::init_and_loop() {
    Ok(()) => {println!("Finished Ok")},
    Err(err) => println!("{}", err),
  }
}


