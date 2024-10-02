extern crate sdl2;

use sdl2::event::Event; // Rust equivalent of C++ using namespace. Last "word" is what you call
use sdl2::pixels::Color; // Like call this with Color:: (you don't actually have to, its just a standard for clarity)

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,  //main render function
          font: &sdl2::ttf::Font, //our chosen font
          window_array: &Vec<Vec<char>>, //3d array for our window
          preview_buffer: &Vec<[i32;2]>,
          clen: i32, rlen: i32,
          current_key: char){ //our dimenions for the canvas

    canvas.set_draw_color(Color::RGB(0, 0, 0)); //set canvas to black
    canvas.clear(); //clears frame allows new one
    let mut rpos = 0; //0,0 is top left
    let mut cpos = 0;
    for x in window_array{
        for grid_char in x{
            if *grid_char != ' '{ //character render
                let render_string = String::from(*grid_char); //character of choice used here
                let font_render = font.render(&render_string); //create a render of the given string
                let font_surface = font_render.solid(Color::RGB(255, 255, 255)).unwrap(); //create a surface out of that render
                let canvas_texture = canvas.texture_creator(); //generate a blank canvas from the canvas 
                let texture = canvas_texture.create_texture_from_surface(font_surface).unwrap(); //copy the font surface onto that texture
                let _ = canvas.copy(
                    &texture,
                    None, //part of texture we want... all of it 
                    sdl2::rect::Rect::new(cpos, rpos, clen as u32, rlen as u32), //first two is where, second is how big
                ).expect("failed copying texture to canvas"); //display that texture to the canvas
            }
            cpos += clen;
        }
        rpos += rlen;
        cpos = 0;
    }

    for buffer_item in preview_buffer{ 
        let render_string = String::from(current_key); //character of choice used here
        let font_render = font.render(&render_string); //create a render of the given string
        let font_surface = font_render.solid(Color::RGB(255, 255, 255)).unwrap(); //create a surface out of that render
        let canvas_texture = canvas.texture_creator(); //generate a blank canvas from the canvas 
        let texture = canvas_texture.create_texture_from_surface(font_surface).unwrap(); //copy the font surface onto that texture
        let _ = canvas.copy(
            &texture,
            None, //part of texture we want... all of it 
            sdl2::rect::Rect::new(buffer_item[1]*clen, buffer_item[0]*rlen, clen as u32, rlen as u32), //first two is where, second is how big
        ).expect("failed copying texture to canvas"); //display that texture to the canvas 
    }

    canvas.present(); //actually commit changes to screen!
}

fn write_buffer(window_array: &mut Vec<Vec<char>>, preview_buffer: &mut Vec<[i32;2]>, current_char: char){
    for buffer_item in &*preview_buffer{
        window_array[buffer_item[0] as usize][buffer_item[1] as usize] = current_char;
    }
    preview_buffer.clear();
}

fn get_mouse_gpos(cpos: i32, rpos: i32, clen: i32, rlen: i32) -> [i32; 2]{
    return [rpos / rlen, cpos / clen]; //converts window dimensions to canvas dimensions
}

fn line_tool(preview_buffer: &mut Vec<[i32; 2]>,
             current_mouse_pos: &[i32; 2], 
             start_mouse_pos: &[i32; 2], 
             clear_buffer: bool){     
    let mut beginx: i32 = start_mouse_pos[0]; //break up into 1 dimension bc lines are 1-dimensional
    let mut beginy: i32 = start_mouse_pos[1];
    let finx: i32 = current_mouse_pos[0];
    let finy: i32 = current_mouse_pos[1];
    
    if clear_buffer{
        preview_buffer.clear();
    }

    let mut x_slope = finx - beginx;
    let mut y_slope = finy - beginy;
    let mut x_iter = 0;
    let mut y_iter = 0;
    if x_slope != 0{
        x_iter = x_slope / x_slope.abs(); 
    }
    if y_slope != 0{
        y_iter = y_slope / y_slope.abs(); 
    }

    x_slope = x_slope.abs();
    y_slope = y_slope.abs();

    let long_slope;
    let short_slope;
    let x_is_long;
    if x_slope > y_slope{
        long_slope = x_slope;
        short_slope = y_slope + 1;
        x_is_long = true;
    }
    else{
        long_slope = y_slope;
        short_slope = x_slope + 1;
        x_is_long = false;
    }

    
    let per_chunk = long_slope / short_slope;
    let mut extra = (long_slope % short_slope) + 1;

    for _ in 0..short_slope{
        let mut this_chunk = per_chunk;
        if extra > 0{
            this_chunk += 1;
            extra -= 1;
        }
        for _ in 0..this_chunk{
            preview_buffer.push([beginx, beginy]);    
            if x_is_long{
                beginx += x_iter;   
            }
            else{
                beginy += y_iter;
            }
        }
        if !x_is_long{
            beginx += x_iter;   
        }
        else{
            beginy += y_iter;
        }
    }
} //commit changes after run


fn rectangle_tool(preview_buffer: &mut Vec<[i32; 2]>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]){ //need to implement preview buffer

    preview_buffer.clear();

    //this is usually top assuming top left to bottom right
    line_tool(preview_buffer,
              &[start_mouse_pos[0], start_mouse_pos[1]],
              &[current_mouse_pos[0], start_mouse_pos[1]],
              false);

    //left
    line_tool(preview_buffer,
              &[start_mouse_pos[0], start_mouse_pos[1]],
              &[start_mouse_pos[0], current_mouse_pos[1]],
              false);

    //bottom
    line_tool(preview_buffer,
              &[start_mouse_pos[0], current_mouse_pos[1]],
              &[current_mouse_pos[0], current_mouse_pos[1]],
              false);

    //right
    line_tool(preview_buffer,
              &[current_mouse_pos[0], start_mouse_pos[1]],
              &[current_mouse_pos[0], current_mouse_pos[1]],
              false);

} //change after this is done running


fn main() {
    
    let window_width: u32 = 1200;
    let window_height: u32 = 800;
    let num_of_cols: u32 = 60; //60
    let num_of_rows: u32 = 40; //40
    let col_length: i32 = (window_width / num_of_cols) as i32;
    let row_length: i32 = (window_height / num_of_rows) as i32;
    let mut preview_buffer: Vec<[i32;2]> = Vec::new();

    let mut window_array = Vec::new();
    for _ in 0..num_of_rows{
        let mut a_row = Vec::new(); 
        for _ in 0..num_of_cols{
            a_row.push(' ');

        }
        window_array.push(a_row);
    }

    let sdl_context = sdl2::init().expect("failed to init sdl");
    let video_subsystem = sdl_context.video().expect("failed to init video subsytem");

    let window = video_subsystem.window("ascii", window_width, window_height) //self explanatory, hereby called "SE" Window builder
        .position_centered()
        .build()
        .expect("failed to build window");

    let mut canvas = window.into_canvas() //SE canvas builder
        .present_vsync()
        .build()
        .expect("failed to build canvas");

    let ttf_context = sdl2::ttf::init().unwrap(); //Maybe add a error message
    let font = ttf_context.load_font("./NotoSansMono-Regular.ttf", 16).unwrap();
        
    video_subsystem.text_input().start();

    let mut event_queue = sdl_context.event_pump().expect("failed to init event queue");
    let mut running = true;
    //decide whether or not to render a new frame (only render when something has a changed)
    let mut render_change = true;
    //holds on to the previous loops gpos so a render doesn't get called if the mouse hasn't moved grid position
    let mut prev_gpos = [-1, -1];
    let mut current_key = 'a';
    let mut keycombo = String::new();
    let mut current_tool = String::from("f");
    let mut mstart_pos = [0, 0];
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
                            if current_tool == String::from("f"){
                                window_array[gpos[0] as usize][gpos[1] as usize] = current_key;
                            }
                            else if current_tool == String::from("r"){
                                mstart_pos = gpos;
                                rectangle_tool(&mut preview_buffer, &gpos, &mstart_pos)
                            }
                            else if current_tool == String::from("l"){
                                mstart_pos = gpos;
                                line_tool(&mut preview_buffer, &gpos, &mstart_pos, true);
                            }
                            prev_gpos = gpos;
                        },
                        _ => {}, //include left-click erase, eventually tool list in match outside
                    }
                    render_change = true;
                },
                Event::MouseMotion {mousestate, x, y, ..} => { //this is for holding down button
                    if mousestate.left(){
                        let gpos = get_mouse_gpos(x, y, col_length, row_length);
                        if current_tool == String::from("f"){
                            window_array[gpos[0] as usize][gpos[1] as usize] = current_key;
                        }
                        else if current_tool == String::from("r"){
                            rectangle_tool(&mut preview_buffer, &gpos, &mstart_pos)
                        }
                        else if current_tool == String::from("l"){
                            line_tool(&mut preview_buffer, &gpos, &mstart_pos, true);
                        }
                        if prev_gpos != gpos{
                            render_change = true;
                        }
                        prev_gpos = gpos;
                    }
                },
                Event::MouseButtonUp {mouse_btn, ..} => {
                    match mouse_btn{
                        sdl2::mouse::MouseButton::Left => {
                            write_buffer(&mut window_array, &mut preview_buffer, current_key);
                        },
                        _ => {}
                    }
                },
                Event::TextInput {text, ..} => {
                    if keycombo.len() > 0{
                        if keycombo == String::from("i"){
                            let text_vec: Vec<char> = text.chars().collect();
                            current_key = text_vec[0];
                            println!("{:?} {}", text, current_key);
                        }
                        else if keycombo == String::from("c"){
                            current_tool = text.to_lowercase();
                        }
                        keycombo = String::from("");
                    }
                    else {
                        if text.to_lowercase() == String::from("i"){
                            keycombo = String::from("i");
                        }
                        else if text.to_lowercase() == String::from("c"){
                            keycombo = String::from("c");
                        }
                    }
                },   
                _ => {},
            }
        }
        if render_change{ //render if change
            render(&mut canvas, &font, &window_array, &preview_buffer, col_length, row_length, current_key);
            render_change = false;
        }
    }
}


