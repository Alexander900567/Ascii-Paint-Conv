extern crate sdl2;

use sdl2::event::Event;

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: &sdl2::ttf::Font){

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();

    let font_render = font.render("a"); //create a render of the given string
    let font_surface = font_render.solid(sdl2::pixels::Color::RGB(255, 255, 255)).unwrap(); //create a surface out of that render
    let canvas_texture = canvas.texture_creator(); //generate a blank canvas from the canvas 
    let texture = canvas_texture.create_texture_from_surface(font_surface).unwrap(); //copy the font surface onto that texture
    canvas.copy(
        &texture,
        None,
        sdl2::rect::Rect::new(0, 0, 50, 50),
    ); //display that texture to the canvas

    canvas.present();
}

fn get_mouse_gpos(cpos: i32, rpos: i32, clen: i32, rlen: i32) -> [i32; 2]{
    return [cpos / clen, rpos / rlen];
}


fn main() {
    
    let window_width: u32 = 1200;
    let window_height: u32 = 800;
    let num_of_cols: u32 = 10;
    let num_of_rows: u32 = 10;
    let col_length: i32 = (window_width / num_of_cols) as i32;
    let row_length: i32 = (window_height / num_of_rows) as i32;
    println!("col_length: {}", col_length);
    println!("row_length: {}", row_length);

    let sdl_context = sdl2::init().expect("failed to init sdl");
    let video_subsystem = sdl_context.video().expect("failed to init video subsytem");

    let window = video_subsystem.window("ascii", window_width, window_height)
        .position_centered()
        .build()
        .expect("failed to build window");

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .expect("failed to build canvas");

    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf", 16).unwrap();
        


    let mut event_queue = sdl_context.event_pump().expect("failed to init event queue");
    let mut running = true;
    while running {
        for event in event_queue.poll_iter(){
            match event{
                Event::Quit {..} => {
                    running = false;
                    break;
                },
                Event::MouseButtonDown {mouse_btn, x, y, ..} => {
                    match mouse_btn{
                        sdl2::mouse::MouseButton::Left => {
                            println!("{:?}", get_mouse_gpos(x, y, col_length, row_length)); 
                        },
                        _ => {},
                    }
                },
                Event::MouseMotion {mousestate, x, y, ..} => {
                    if mousestate.left(){
                        println!("{:?}", get_mouse_gpos(x, y, col_length, row_length));
                    }
                },
                _ => {},
            }
        }
        render(&mut canvas, &font);
    }
}


