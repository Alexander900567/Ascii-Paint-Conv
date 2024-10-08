extern crate sdl2;

use sdl2::event::Event; // Rust equivalent of C++ using namespace. Last "word" is what you call
use sdl2::pixels::Color; // Like call this with Color:: (you don't actually have to, its just a standard for clarity)

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,  //main render function
          font: &sdl2::ttf::Font, //our chosen font
          window_array: &Vec<Vec<char>>, //3d array for our window
          preview_buffer: &Vec<[i32;2]>,
          current_key: char){ //our dimenions for the canvas

    let mut render_array = window_array.clone();
    
    for buffer_item in preview_buffer{ 
        render_array[buffer_item[0] as usize][buffer_item[1] as usize] = current_key;
    }

    canvas.set_draw_color(Color::RGB(0, 0, 0)); //set canvas to black
    canvas.clear(); //clears frame allows new one
    
    let mut array_string = String::new();
    for x in &render_array{
        for grid_char in x{
            array_string.push(*grid_char);
        }
        array_string.push('\n');
    }

    let font_render = font.render(&array_string); //create a render of the given string
    let font_surface = font_render.blended_wrapped(Color::RGB(255, 255, 255), 0).unwrap(); //create a surface out of that render
    let canvas_texture = canvas.texture_creator(); //generate a blank canvas from the canvas 
    let texture = canvas_texture.create_texture_from_surface(font_surface).unwrap(); //copy the font surface onto that texture
    let _ = canvas.copy(
        &texture,
        None, //part of texture we want... all of it 
        sdl2::rect::Rect::new(0, 0, 1200, 800), //first two is where, second is how big
    ).expect("failed copying texture to canvas"); //display that texture to the canvas


    canvas.present(); //actually commit changes to screen!
}

fn write_buffer(window_array: &mut Vec<Vec<char>>, preview_buffer: &mut Vec<[i32;2]>, current_char: char){
    for buffer_item in &*preview_buffer{
        window_array[buffer_item[0] as usize][buffer_item[1] as usize] = current_char;
    }
    preview_buffer.clear();
}
//consider:
//adding backspace compatibility
//adding delete compatibility (clears buffer?)

fn copy_to_clipboard(window_array: &Vec<Vec<char>>, clipboard: &sdl2::clipboard::ClipboardUtil){
    let mut array_string = String::new();
    for x in window_array{
        for grid_char in x{
            array_string.push(*grid_char);
        }
        array_string.push('\n');
    }
    let _ = clipboard.set_clipboard_text(&array_string).expect("Failed to copy to clipboard");
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

fn rectangle_tool(preview_buffer: &mut Vec<[i32; 2]>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]){

    preview_buffer.clear();
    //4 lines
    line_tool(preview_buffer,
              &[start_mouse_pos[0], start_mouse_pos[1]], //s,s to c,s
              &[current_mouse_pos[0], start_mouse_pos[1]], //top left to bottom left
              false);
    line_tool(preview_buffer,
              &[start_mouse_pos[0], start_mouse_pos[1]], //s,s to s,c
              &[start_mouse_pos[0], current_mouse_pos[1]], //top left to top right
              false);
    line_tool(preview_buffer,
              &[start_mouse_pos[0], current_mouse_pos[1]], //s,c to c,c
              &[current_mouse_pos[0], current_mouse_pos[1]], //top right to bottom right
              false);
    line_tool(preview_buffer,
              &[current_mouse_pos[0], start_mouse_pos[1]], //c,s to c,c
              &[current_mouse_pos[0], current_mouse_pos[1]], //bottom left to bottom right
              false);
}

fn filled_rectangle_tool(preview_buffer: &mut Vec<[i32; 2]>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {

    preview_buffer.clear();

    let beginx: i32 = start_mouse_pos[0];
    let mut beginy: i32 = start_mouse_pos[1];
    let finx: i32 = current_mouse_pos[0];
    let finy: i32 = current_mouse_pos[1];
    
    let smallx:i32;
    let bigx:i32;

    if beginx <= finx {
        smallx = beginx;
        bigx = finx;
    }
    else {
        smallx = finx;
        bigx = beginx;
    }
    
    if beginy <= finy {
    for _ in beginy..=finy {
        line_tool(preview_buffer,
        &[bigx, beginy],
        &[smallx, beginy],
        false);
        beginy += 1;
        }
    }   
}

fn circle_tool(preview_buffer: &mut Vec<[i32; 2]>,
    current_mouse_pos: &[i32; 2],
    start_mouse_pos: &[i32; 2],
    clear_buffer: bool) { //this is faster when ellipse is circle
    
        if clear_buffer{
            preview_buffer.clear();
        }
// Uses the [Midpoint Ellipse Drawing Algorithm](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/).
// (Modified from Bresenham's algorithm)
    let beginx: i32 = start_mouse_pos[0];
    let finx: i32 = current_mouse_pos[0];
    let beginy: i32 = start_mouse_pos[1];
    let finy: i32 = current_mouse_pos[1];

    let x_component:i32 = finx - beginx;
    let y_component:i32 = finy - beginy;
    let r:i32;
    let r0:f32 = f32::sqrt((x_component as f32 * x_component as f32) + (y_component as f32 * y_component as f32)); //REAL hypotenuse length
    match(x_component, y_component) {
        (x,y) if x != 0 && y != 0 => {
        let o:i32 = y_component.abs(); //to keep scalar factor positive
        let r0:f32 = r0/(2f32 * f32::sin(o as f32/r0));
        r = r0.floor() as i32; //radius converted to int to work with buffer vector
        }, 
        (0,0) => {
            r = r0.floor() as i32;
        },
        _ => r = 0
    }

    let mut x:i32 = 0i32;
    let mut y:i32 = r;
    let mut p:i32 = 1 - r;

    while x <= y {
        preview_buffer.push([(beginx + x), (beginy + y)]);
        preview_buffer.push([(beginx + y), (beginy + x)]);
        preview_buffer.push([(beginx - y), (beginy + x)]);
        preview_buffer.push([(beginx - x), (beginy + y)]);
        preview_buffer.push([(beginx - x), (beginy - y)]);
        preview_buffer.push([(beginx - y), (beginy - x)]);
        preview_buffer.push([(beginx + y), (beginy - x)]);
        preview_buffer.push([(beginx + x), (beginy - y)]);

        x += 1;
        if p < 0 {
            p += 2 * x + 1
        }
        else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }    
}


fn text_tool(window_array: &mut Vec<Vec<char>>, &prev_gpos: &[i32;2], input: &String, num_of_col: u32) -> [i32;2]{
    let text_vec: Vec<char> = input.chars().collect();
    if prev_gpos[1] >= (num_of_col as i32){
        window_array[prev_gpos[0] as usize][(num_of_col - 1) as usize] = text_vec[0];
    }
    else{
        window_array[prev_gpos[0] as usize][prev_gpos[1] as usize] = text_vec[0];
    }
    return [prev_gpos[0], prev_gpos[1] + 1];
}

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
    let clipboard = video_subsystem.clipboard();
        
    video_subsystem.text_input().start();

    /////////
    let mut times = Vec::new();
    ////


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
                        sdl2::mouse::MouseButton::Left => { //Keybinds
                            let gpos = get_mouse_gpos(x, y, col_length, row_length);
                            if &current_tool == "f"{
                                window_array[gpos[0] as usize][gpos[1] as usize] = current_key;
                            }
                            else if &current_tool == "l"{
                                mstart_pos = gpos;
                                line_tool(&mut preview_buffer, &gpos, &mstart_pos, true);
                            }
                            else if &current_tool == "r"{
                                mstart_pos = gpos;
                                rectangle_tool(&mut preview_buffer, &gpos, &mstart_pos)
                            }
                            else if &current_tool == "s"{
                                mstart_pos = gpos;
                                filled_rectangle_tool(&mut preview_buffer, &gpos, &mstart_pos);
                            }
                            else if &current_tool == "o"{
                                mstart_pos = gpos;
                                circle_tool(&mut preview_buffer, &gpos, &mstart_pos, true);
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
                        if &current_tool == "f"{
                            window_array[gpos[0] as usize][gpos[1] as usize] = current_key;
                        }
                        else if &current_tool == "l"{
                            line_tool(&mut preview_buffer, &gpos, &mstart_pos, true);
                        }
                        else if &current_tool == "r"{
                            rectangle_tool(&mut preview_buffer, &gpos, &mstart_pos)
                        }
                        else if &current_tool == "s"{
                            filled_rectangle_tool(&mut preview_buffer, &gpos, &mstart_pos)
                        }
                        else if &current_tool == "o"{
                            circle_tool(&mut preview_buffer, &gpos, &mstart_pos, true);
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
                            render_change = true;
                        },
                        _ => {}
                    }
                },
                Event::TextInput {text, ..} => {
                    println!("text: {}", text);
                    if &current_tool == "t"{
                        prev_gpos = text_tool(&mut window_array, &prev_gpos, &text, num_of_cols);
                        render_change = true;
                    }
                    else if keycombo.len() > 0{
                        if &keycombo == "i"{
                            let text_vec: Vec<char> = text.chars().collect();
                            current_key = text_vec[0];
                        }
                        else if &keycombo == "c"{
                            current_tool = text.to_lowercase();
                        }
                        keycombo = String::from("");
                    }
                    else {
                        if &(text.to_lowercase()) == "i"{
                            keycombo = String::from("i");
                        }
                        else if &(text.to_lowercase()) == "c"{
                            keycombo = String::from("c");
                        }
                        else if &(text.to_lowercase()) == "b"{
                            copy_to_clipboard(&window_array, &clipboard);
                        }
                    }
                },   
                Event::KeyUp {keycode, ..} =>{
                    match keycode {
                        Some(sdl2::keyboard::Keycode::ESCAPE) =>{
                            if &current_tool == "t"{
                                current_tool = String::from("f");
                            }
                        }
                        _ => {}
                    }
                }
                _ => {},
            }
        }
        if render_change{ //render if change
            let pre = std::time::SystemTime::now();
            render(&mut canvas, &font, &window_array, &preview_buffer, current_key);
            render_change = false;
            let post = std::time::SystemTime::now();
            times.push(post.duration_since(pre).unwrap().as_secs_f64());
        }
    }

    println!("Average render from {} renders", times.len());
    let mut sum: f64 = 0.0;
    for x in &times{
        sum += x;
    }

    println!("{}", sum / times.len() as f64);
}


