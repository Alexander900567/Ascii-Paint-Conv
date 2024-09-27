extern crate sdl2;

use sdl2::event::Event;

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, 
          font: &sdl2::ttf::Font, 
          window_array: &Vec<Vec<char>>,
          clen: i32, rlen: i32){
    
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear(); 
    let mut rpos = 0;
    let mut cpos = 0;
    for x in window_array{
        for grid_char in x{
            if *grid_char != ' '{
                let render_string = String::from(*grid_char); 
                let font_render = font.render(&render_string); //create a render of the given string
                let font_surface = font_render.solid(sdl2::pixels::Color::RGB(255, 255, 255)).unwrap(); //create a surface out of that render
                let canvas_texture = canvas.texture_creator(); //generate a blank canvas from the canvas 
                let texture = canvas_texture.create_texture_from_surface(font_surface).unwrap(); //copy the font surface onto that texture
                let _ = canvas.copy(
                    &texture,
                    None,
                    sdl2::rect::Rect::new(cpos, rpos, clen as u32, rlen as u32),
                ).expect("failed copying texture to canvas"); //display that texture to the canvas
            }
            cpos += clen;
        }
        rpos += rlen;
        cpos = 0;
    }

    canvas.present();
}

fn get_mouse_gpos(cpos: i32, rpos: i32, clen: i32, rlen: i32) -> [i32; 2]{
    return [rpos / rlen, cpos / clen];
}


fn main() {
    
    let window_width: u32 = 1200;
    let window_height: u32 = 800;
    let num_of_cols: u32 = 60;
    let num_of_rows: u32 = 40;
    let col_length: i32 = (window_width / num_of_cols) as i32;
    let row_length: i32 = (window_height / num_of_rows) as i32;
    println!("col_length: {}", col_length);
    println!("row_length: {}", row_length);

    let mut window_array = Vec::new();
    for _ in 0..num_of_rows{
        let mut a_row = Vec::new(); 
        for _ in 0..num_of_cols{
            a_row.push(' ');

        }
        window_array.push(a_row);
    }


    for x in &window_array{
        println!("{:?}", x);
    }


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
    let font = ttf_context.load_font("./NotoSansMono-Regular.ttf", 16).unwrap();
        


    let mut event_queue = sdl_context.event_pump().expect("failed to init event queue");
    let mut running = true;
    let mut render_change = true;
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
                            let gpos = get_mouse_gpos(x, y, col_length, row_length);
                            //println!("{:?}", gpos);
                            window_array[gpos[0] as usize][gpos[1] as usize] = 'a';
                        },
                        _ => {},
                    }
                    render_change = true;
                },
                Event::MouseMotion {mousestate, x, y, ..} => {
                    if mousestate.left(){
                        let gpos = get_mouse_gpos(x, y, col_length, row_length);
                        //println!("{:?}", gpos);
                        window_array[gpos[0] as usize][gpos[1] as usize] = 'a';
                        render_change = true
                        /*
                        for x in &window_array{
                            println!("{:?}", x);
                        }
                        println!("------------")
                        */
                    }
                },
                _ => {},
            }
        }
        if render_change{
            render(&mut canvas, &font, &window_array, col_length, row_length);
            render_change = false;
        }
    }
}


