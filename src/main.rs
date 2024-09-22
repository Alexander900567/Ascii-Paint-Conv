extern crate sdl2;

fn main() {
    
    let window_width: u32 = 600;
    let window_height: u32 = 800;

    let sdl_context = sdl2::init().expect("failed to init sdl");
    let video_subsystem = sdl_context.video().expect("failed to init video subsytem");

    let window = video_subsystem.window("ascii", window_width, window_height)
        .position_centered()
        .build()
        .expect("failed to build window");

}


