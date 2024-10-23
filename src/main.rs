extern crate sdl2;
extern crate image;
extern crate rayon;
mod image_conv;

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
    
    let mut array_string = String::new(); //makes our grid a string, so we can write, copy, etc.
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

fn write_buffer(window_array: &mut Vec<Vec<char>>, preview_buffer: &mut Vec<[i32;2]>, current_char: char) {
    for buffer_item in &*preview_buffer{
        window_array[buffer_item[0] as usize][buffer_item[1] as usize] = current_char;
    }
    preview_buffer.clear();
}
//consider:
//adding backspace compatibility
//adding delete compatibility (clears buffer?)

fn copy_to_clipboard(window_array: &Vec<Vec<char>>, clipboard: &sdl2::clipboard::ClipboardUtil) {
    let mut array_string = String::new();
    for x in window_array {
        for grid_char in x{
            array_string.push(*grid_char);
        }
        array_string.push('\n');
    }
    let _ = clipboard.set_clipboard_text(&array_string).expect("Failed to copy to clipboard");
}


fn get_mouse_gpos(cpos: i32, rpos: i32, clen: i32, rlen: i32, num_of_cols: u32, num_of_rows: u32) -> [i32; 2] {
    let mut rgpos: i32 = rpos / rlen; //row global position, row position, row length
    let mut cgpos: i32 = cpos / clen; // same but column
    let rnumi = num_of_rows as i32;
    let cnumi = num_of_cols as i32;

    if rgpos < 0 {rgpos = 0;} //sets 0 as left bound
    else if rgpos >= rnumi {rgpos = rnumi - 1;} //right bound
    if cgpos < 0 {cgpos = 0;} //upper bound
    else if cgpos >= cnumi {cgpos = cnumi - 1;} //lower bound

    return [rgpos, cgpos]; //converts window dimensions to canvas dimensions
}

fn line_tool(preview_buffer: &mut Vec<[i32; 2]>,
             current_mouse_pos: &[i32; 2], 
             start_mouse_pos: &[i32; 2], 
             clear_buffer: bool) {     
    let mut beginx: i32 = start_mouse_pos[0]; //we do this a lot, but we are essentially just shorthanding these vars
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

    if x_slope != 0 {
        x_iter = x_slope / x_slope.abs(); 
    }
    if y_slope != 0 {
        y_iter = y_slope / y_slope.abs(); 
    }

    x_slope = x_slope.abs();
    y_slope = y_slope.abs();

    let long_slope;
    let short_slope;
    let x_is_long;
    if x_slope > y_slope {
        long_slope = x_slope;
        short_slope = y_slope + 1;
        x_is_long = true;
    }
    else {
        long_slope = y_slope;
        short_slope = x_slope + 1;
        x_is_long = false;
    }

    let per_chunk = long_slope / short_slope;
    let mut extra = (long_slope % short_slope) + 1;

    for _ in 0..short_slope {
        let mut this_chunk = per_chunk;
        if extra > 0 {
            this_chunk += 1;
            extra -= 1;
        }
        for _ in 0..this_chunk {
            preview_buffer.push([beginx, beginy]);    
            if x_is_long {
                beginx += x_iter;   
            }
            else {
                beginy += y_iter;
            }
        }
        if !x_is_long {
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
              &[start_mouse_pos[0], start_mouse_pos[1]], //(s,s) to (c,s)
              &[current_mouse_pos[0], start_mouse_pos[1]], //top left to bottom left
              false);
    line_tool(preview_buffer,
              &[start_mouse_pos[0], start_mouse_pos[1]], //(s,s) to (s,c)
              &[start_mouse_pos[0], current_mouse_pos[1]], //top left to top right
              false);
    line_tool(preview_buffer,
              &[start_mouse_pos[0], current_mouse_pos[1]], //(s,c) to (c,c)
              &[current_mouse_pos[0], current_mouse_pos[1]], //top right to bottom right
              false);
    line_tool(preview_buffer,
              &[current_mouse_pos[0], start_mouse_pos[1]], //(c,s) to (c,c)
              &[current_mouse_pos[0], current_mouse_pos[1]], //bottom left to bottom right
              false);
}

fn filled_rectangle_tool(preview_buffer: &mut Vec<[i32; 2]>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {

    preview_buffer.clear(); //clears previous preview, so we can load new one

    let beginx: i32 = start_mouse_pos[0];
    let beginy: i32 = start_mouse_pos[1];
    let finx: i32 = current_mouse_pos[0];
    let finy: i32 = current_mouse_pos[1];
    
    let leftx:i32;
    let rightx:i32;

    if beginx <= finx { //right quadrants case
        leftx = beginx;
        rightx = finx;
    }
    else { //left quadrants case
        leftx = finx;
        rightx = beginx;
    }

    for x in leftx..=rightx { //iterates vertical lines
        line_tool(preview_buffer,
        &[x, beginy], //further iterates those lines horizontally (left to right or right to left)
        &[x, finy],
        false);
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
// (Modified from Bresenham's algorithm) <- These are the credits given by the Rust imageproc conics functions.
//This is just a modified draw_hollow_circle
    let beginx: i32 = start_mouse_pos[0];
    let finx: i32 = current_mouse_pos[0];
    let beginy: i32 = start_mouse_pos[1];
    let finy: i32 = current_mouse_pos[1];

    let x_component:i32 = finx - beginx;
    let y_component:i32 = finy - beginy;
    let r:i32;
    let diagonal_r:f32 = f32::sqrt((x_component as f32 * x_component as f32) + (y_component as f32 * y_component as f32)); //pythag h
    //theory: given r = 10
    /* diagonal_r = real hypotenuse = 10*sqrt(2) = r*ratio
    r (radius of circle)= diagonal_r / ratio
    ratio = hypotenuse of a triangle with sides divided by r with same angle theta = h = (o/r)/sin(theta) 
    theta = sin^-1(o/diagonal_r) */
    match(x_component, y_component) {
        (x,y) if x != 0 && y != 0 => { //non-cardinal case
        let o:i32 = y_component.abs(); //to keep scalar factor positive (since we're about to use sin)
        let angle_theta:f32 = f32::asin(o as f32/diagonal_r);
        let h:f32 = (o as f32/diagonal_r as f32)/f32::sin(angle_theta);
        let r0: f32 = diagonal_r/h;
        r = r0.floor() as i32; //radius converted to int to work with buffer vector
        }, 
        (0, _) => { //y-axis cardinal case
            let r0: f32 = diagonal_r;
            r = r0.floor() as i32;
        },
        (_, 0) => { //x-axis cardinal case
            let r0: f32 = diagonal_r;
            r = r0.floor() as i32;
        },
        _ => r = 0 //catch all
    }

    let mut x:i32 = 0i32;
    let mut y:i32 = r; //(x,y) = (0,r)
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

        x += 1; //all 4 regions
        if p < 0 {
            p += 2 * x + 1
        }
        else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }    
}

fn filled_circle_tool (preview_buffer: &mut Vec<[i32; 2]>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2], clear_buffer: bool) { //basically, just fill in line tools, trig if necessary
    //same credits as above, just draw_filled_circle modified

    if clear_buffer{
        preview_buffer.clear();
    }

    let beginx: i32 = start_mouse_pos[0];
    let finx: i32 = current_mouse_pos[0];
    let beginy: i32 = start_mouse_pos[1];
    let finy: i32 = current_mouse_pos[1];

    let x_component:i32 = finx - beginx;
    let y_component:i32 = finy - beginy;
    let r:i32;
    let diagonal_r:f32 = f32::sqrt((x_component as f32 * x_component as f32) + (y_component as f32 * y_component as f32));

    match(x_component, y_component) {
        (x,y) if x != 0 && y != 0 => { //non-cardinal case
        let o:i32 = y_component.abs(); //to keep scalar factor positive (since we're about to use sin)
        let angle_theta:f32 = f32::asin(o as f32/diagonal_r);
        let h:f32 = (o as f32/diagonal_r as f32)/f32::sin(angle_theta);
        let r0: f32 = diagonal_r/h;
        r = r0.floor() as i32; //radius converted to int to work with buffer vector
        }, 
        (0, _) => { //y-axis cardinal case
            let r0: f32 = diagonal_r;
            r = r0.floor() as i32;
        },
        (_, 0) => { //x-axis cardinal case
            let r0: f32 = diagonal_r;
            r = r0.floor() as i32;
        },
        _ => r = 0 //catch all
    }

    let mut x = 0i32;
    let mut y = r;
    let mut p = 1 - r; //haven't assigned r, assign later

    while x <= y {

        line_tool(preview_buffer,
        &[(beginx + x), (beginy + y)], 
        &[(beginx - x), (beginy + y)],
        false);

        line_tool(preview_buffer,
        &[(beginx + y), (beginy + x)], 
        &[(beginx - y), (beginy + x)],
        false);

        line_tool(preview_buffer,
        &[(beginx + x), (beginy - y)], 
        &[(beginx - x), (beginy - y)],
        false);

        line_tool(preview_buffer,
        &[(beginx + y), (beginy - x)], 
        &[(beginx - y), (beginy - x)],
        false);

        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        }
        else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}

fn text_tool(window_array: &mut Vec<Vec<char>>, &prev_gpos: &[i32;2], input: &String, num_of_cols: u32) -> [i32;2] {
    let text_vec: Vec<char> = input.chars().collect();
    if prev_gpos[1] >= (num_of_cols as i32) {
        window_array[prev_gpos[0] as usize][(num_of_cols - 1) as usize] = text_vec[0];
    }
    else {
        window_array[prev_gpos[0] as usize][prev_gpos[1] as usize] = text_vec[0];
    }
    return [prev_gpos[0], std::cmp::min(prev_gpos[1]+1, (num_of_cols as i32)-1)];
}

fn main() {
    
    let window_width: u32 = 1200; //1200x800 screen
    let window_height: u32 = 800;
    let num_of_cols: u32 = 60; //60x40
    let num_of_rows: u32 = 40;
    let col_length: i32 = (window_width / num_of_cols) as i32;
    let row_length: i32 = (window_height / num_of_rows) as i32;
    let mut preview_buffer: Vec<[i32;2]> = Vec::new();

    let mut window_array = Vec::new();
    for _ in 0..num_of_rows{
        let mut a_row = Vec::new(); 
        for _ in 0..num_of_cols{
            a_row.push(' '); //populate with spaces
        }
        window_array.push(a_row);
    }

    let sdl_context = sdl2::init().expect("failed to init sdl");
    let video_subsystem = sdl_context.video().expect("failed to init video subsytem");

    let window = video_subsystem.window("ascii", window_width, window_height) //builds and names window
        .position_centered()
        .build()
        .expect("failed to build window");

    let mut canvas = window.into_canvas() //builds canvas
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
    //holds on to the previous loops' gpos so a render doesn't get called if the mouse hasn't moved grid position
    let mut prev_gpos = [0, 0];
    let mut current_key = 'a'; //default char is 'a'
    let mut keycombo = String::new(); //will hold our key commands
    let mut current_tool = String::from("f"); //default "f" because c + f is our paint tool
    let mut tool_modifier = Vec::from([String::from(" "), String::from(" ")]);
    let mut mstart_pos = [0, 0];
    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                    break; //graceful quit
                },
                Event::MouseButtonDown {mouse_btn, x, y, ..} => { //initial click
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => { //Keybinds
                            let gpos = get_mouse_gpos(x, y, col_length, row_length, num_of_cols, num_of_rows);
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
                            else if &current_tool == "q"{
                                mstart_pos = gpos;
                                filled_circle_tool(&mut preview_buffer, &gpos, &mstart_pos, true);
                            }
                            else if &current_tool == "p"{
                                mstart_pos = gpos;
                                rectangle_tool(&mut preview_buffer, &gpos, &mstart_pos)
                            }
                            prev_gpos = gpos;
                        },
                        _ => {}, //eventually will be replaced with a tool list
                    }
                    render_change = true;
                },
                Event::MouseMotion {mousestate, x, y, ..} => { //this is for holding down button
                    if mousestate.left(){
                        let gpos = get_mouse_gpos(x, y, col_length, row_length, num_of_cols, num_of_rows);
                        if &current_tool == "f"{ //gives these functions the needed parameters
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
                        else if &current_tool == "q"{
                            filled_circle_tool(&mut preview_buffer, &gpos, &mstart_pos, true);
                        }
                        else if &current_tool == "p"{
                            rectangle_tool(&mut preview_buffer, &gpos, &mstart_pos)
                        }
                        if prev_gpos != gpos{
                            render_change = true;
                        }
                        prev_gpos = gpos;
                    }
                },
                Event::MouseButtonUp {mouse_btn, x, y, ..} => { //let go
                    match mouse_btn{
                        sdl2::mouse::MouseButton::Left => {
                            if &current_tool == "p"{
                                let gpos = get_mouse_gpos(x, y, col_length, row_length, num_of_cols, num_of_rows);
                                image_conv::convert_image_put_in_window(&mut window_array, &gpos, &mstart_pos, &tool_modifier[0], &tool_modifier[1]); 
                                preview_buffer.clear();
                            }
                            else{
                                write_buffer(&mut window_array, &mut preview_buffer, current_key);
                            }
                            render_change = true;
                        },
                        _ => {}
                    }
                },
                Event::TextInput {text, ..} => { //keyboard determines keycombo (keybinds)
                    println!("text: {}", text);
                    if &current_tool == "t"{
                        prev_gpos = text_tool(&mut window_array, &prev_gpos, &text, num_of_cols); //text mode case
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
                        else if &keycombo == "m"{
                            if &current_tool == "p" && &(text.to_lowercase()) == "l"{ 
                                if &tool_modifier[1] == " " {tool_modifier[1] = String::from("l");}
                                else {tool_modifier[1] = String::from(" ");}
                            }
                            else{
                                tool_modifier[0] = text.to_lowercase();
                            }
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
                        else if &(text.to_lowercase()) == "m"{
                            keycombo = String::from("m");
                        }
                    }
                },   
                Event::KeyUp {keycode, ..} =>{
                    if &current_tool == "t"{
                        match keycode {
                            Some(sdl2::keyboard::Keycode::ESCAPE) =>{ //leave text mode
                                current_tool = String::from("f");
                            }
                            Some(sdl2::keyboard::Keycode::BACKSPACE) => { //backspace! finally! what is this? 2024? are we sure it isn't 3024?
                                let mut end_offset = 0;
                                if prev_gpos[1] == (num_of_cols as i32) - 1 && //updates our position
                                window_array[prev_gpos[0] as usize][prev_gpos[1] as usize] != ' '{ //moves to that new position
                                    end_offset = 1;
                                }
                                window_array[prev_gpos[0] as usize][(std::cmp::max(prev_gpos[1]-1+end_offset, 0)) as usize] = ' ';
                                prev_gpos = [prev_gpos[0], std::cmp::max(prev_gpos[1] - 1 + end_offset, 0)];
                                render_change = true;
                            }
                            Some(sdl2::keyboard::Keycode::UP) => { //directions??? No way, it's 4024
                                prev_gpos = [std::cmp::max(prev_gpos[0]-1, 0), prev_gpos[1]];
                            }
                            Some(sdl2::keyboard::Keycode::DOWN) => {
                                prev_gpos = [std::cmp::min(prev_gpos[0]+1, (num_of_rows as i32)-1), prev_gpos[1]];
                            }
                            Some(sdl2::keyboard::Keycode::LEFT) => {
                                prev_gpos = [prev_gpos[0], std::cmp::max(prev_gpos[1]-1, 0)];
                            }
                            Some(sdl2::keyboard::Keycode::RIGHT) => {
                                prev_gpos = [prev_gpos[0], std::cmp::min(prev_gpos[1]+1, (num_of_cols as i32)-1)];
                            }
                            _ => {}
                        }   
                    }
                },
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

    println!("Average render from {} renders", times.len()); //gives us stats, so we know if program is slow
    let mut sum: f64 = 0.0;
    for x in &times {
        sum += x; //total time
    }

    println!("{}", sum / times.len() as f64); //computes average
}
