extern crate sdl2;
extern crate image;
extern crate rayon;
mod image_conv;
mod main_window;
mod undo_redo;

use sdl2::event::Event; // Rust equivalent of C++ using namespace. Last "word" is what you call

fn line_tool(main_window: &mut main_window::MainWindow<'_>,
             current_mouse_pos: &[i32; 2], 
             start_mouse_pos: &[i32; 2], 
             clear_buffer: bool) {     
    let mut begin_row: i32 = start_mouse_pos[0]; //we do this a lot, but we are essentially just shorthanding these vars
    let mut begin_col: i32 = start_mouse_pos[1];
    let fin_row: i32 = current_mouse_pos[0];
    let fin_col: i32 = current_mouse_pos[1];
    
    if clear_buffer{
        main_window.preview_buffer.clear();
    }

    let mut x_slope = fin_row - begin_row;
    let mut y_slope = fin_col - begin_col;
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
            main_window.add_to_preview_buffer(begin_row, begin_col);    
            if x_is_long {
                begin_row += x_iter;   
            }
            else {
                begin_col += y_iter;
            }
        }
        if !x_is_long {
            begin_row += x_iter;   
        }
        else{
            begin_col += y_iter;
        }
    }
} //commit changes after run

fn rectangle_tool(main_window: &mut main_window::MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]){

    main_window.preview_buffer.clear();
    //4 lines
    line_tool(main_window,
              &[start_mouse_pos[0], start_mouse_pos[1]], //(s,s) to (c,s)
              &[current_mouse_pos[0], start_mouse_pos[1]], //top left to bottom left
              false);
    line_tool(main_window,
              &[start_mouse_pos[0], start_mouse_pos[1]], //(s,s) to (s,c)
              &[start_mouse_pos[0], current_mouse_pos[1]], //top left to top right
              false);
    line_tool(main_window,
              &[start_mouse_pos[0], current_mouse_pos[1]], //(s,c) to (c,c)
              &[current_mouse_pos[0], current_mouse_pos[1]], //top right to bottom right
              false);
    line_tool(main_window,
              &[current_mouse_pos[0], start_mouse_pos[1]], //(c,s) to (c,c)
              &[current_mouse_pos[0], current_mouse_pos[1]], //bottom left to bottom right
              false);
}

fn filled_rectangle_tool(main_window: &mut main_window::MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {

    main_window.preview_buffer.clear(); //clears previous preview, so we can load new one

    let begin_row: i32 = start_mouse_pos[0];
    let begin_col: i32 = start_mouse_pos[1];
    let fin_row: i32 = current_mouse_pos[0];
    let fin_col: i32 = current_mouse_pos[1];
    
    let leftx:i32;
    let rightx:i32;

    if begin_row <= fin_row { //right quadrants case
        leftx = begin_row;
        rightx = fin_row;
    }
    else { //left quadrants case
        leftx = fin_row;
        rightx = begin_row;
    }

    for x in leftx..=rightx { //iterates vertical lines
        line_tool(main_window,
        &[x, begin_col], //further iterates those lines horizontally (left to right or right to left)
        &[x, fin_col],
        false);
    }
}

fn circle_tool(main_window: &mut main_window::MainWindow<'_>,
    current_mouse_pos: &[i32; 2],
    start_mouse_pos: &[i32; 2]) { //this is faster when ellipse is circle

    main_window.preview_buffer.clear();
// Uses the [Midpoint Ellipse Drawing Algorithm](https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/).
// (Modified from Bresenham's algorithm) <- These are the credits given by the Rust imageproc conics functions.
//This is just a modified draw_hollow_circle
    let begin_row: i32 = start_mouse_pos[0];
    let fin_row: i32 = current_mouse_pos[0];
    let begin_col: i32 = start_mouse_pos[1];
    let fin_col: i32 = current_mouse_pos[1];

    let x_component:i32 = fin_row - begin_row;
    let y_component:i32 = fin_col - begin_col;
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
        main_window.add_to_preview_buffer(begin_row + x, begin_col + y);
        main_window.add_to_preview_buffer(begin_row + y, begin_col + x);
        main_window.add_to_preview_buffer(begin_row - y, begin_col + x);
        main_window.add_to_preview_buffer(begin_row - x, begin_col + y);
        main_window.add_to_preview_buffer(begin_row - x, begin_col - y);
        main_window.add_to_preview_buffer(begin_row - y, begin_col - x);
        main_window.add_to_preview_buffer(begin_row + y, begin_col - x);
        main_window.add_to_preview_buffer(begin_row + x, begin_col - y);

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

fn filled_circle_tool(main_window: &mut main_window::MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) { //basically, just fill in line tools, trig if necessary
    //same credits as above, just draw_filled_circle modified

    main_window.preview_buffer.clear();

    let begin_row: i32 = start_mouse_pos[0];
    let fin_row: i32 = current_mouse_pos[0];
    let begin_col: i32 = start_mouse_pos[1];
    let fin_col: i32 = current_mouse_pos[1];

    let x_component:i32 = fin_row - begin_row;
    let y_component:i32 = fin_col - begin_col;
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

        line_tool(main_window,
        &[(begin_row + x), (begin_col + y)], 
        &[(begin_row - x), (begin_col + y)],
        false);

        line_tool(main_window,
        &[(begin_row + y), (begin_col + x)], 
        &[(begin_row - y), (begin_col + x)],
        false);

        line_tool(main_window,
        &[(begin_row + x), (begin_col - y)], 
        &[(begin_row - x), (begin_col - y)],
        false);

        line_tool(main_window,
        &[(begin_row + y), (begin_col - x)], 
        &[(begin_row - y), (begin_col - x)],
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

fn draw_ellipse<F>(main_window: &mut main_window::MainWindow<'_>, mut render_func: F, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) //necessary for ellipse_tool and filled_ellipse_tool
    //same credits (docs.rs) but draw_ellipse ofc
    //this func is the meat for drawing the ellipse, ellipse_tool and filled_ellipse tool just call specialized versions of this
    where
    F: FnMut(&mut main_window::MainWindow<'_>, i32, i32, i32, i32), {

    let begin_row: i32 = start_mouse_pos[0];
    let fin_row: i32 = current_mouse_pos[0];
    let begin_col: i32 = start_mouse_pos[1];
    let fin_col: i32 = current_mouse_pos[1];

    let x_component:i32 = (fin_row - begin_row).abs();
    let y_component:i32 = (fin_col - begin_col).abs();
    let x_squared: f32 = (x_component * x_component) as f32;
    let y_squared: f32 = (y_component * y_component) as f32;

    let mut x: i32 = 0;
    let mut y: i32 = y_component;

    let mut px:f32 = 0.0;
    let mut py:f32 = 2.0 * x_squared * y as f32;

    render_func(main_window, begin_row, begin_col, x, y);

    //Top and bottom
    let mut p:f32 = y_squared - (x_squared * y_component as f32) + (0.25f32 * x_squared);
    while px <= py {
        x += 1;
        px += 2.0 * y_squared;
        if p < 0.0 {
            p += y_squared + px;
        }
        else {
            y -= 1;
            py += -2.0 * x_squared;
            p += y_squared + px - py;
        }
        render_func(main_window, begin_row, begin_col, x, y);
    }

    //Left and right
    p = (y_squared * ((x as f32 + 0.5).powi(2))) + (x_squared * (y - 1).pow(2) as f32) - (x_squared * y_squared);
    while y >= 0 {
        y -= 1;
        py += -2.0 * x_squared;
        if p > 0.0 {
            p += x_squared - py;
        }
        else {
            x += 1;
            px += (2.0 * y_squared);
            p += x_squared - py + px;
        }
        render_func(main_window, begin_row, begin_col, x, y);
    }
}

fn ellipse_tool(main_window: &mut main_window::MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {
    //docs.rs draw_hollow_ellipse_mut

    main_window.preview_buffer.clear();

    let begin_row: i32 = start_mouse_pos[0]; //noted, will rename, nvm now, but only this section for now [0] is row, right?
    let fin_row: i32 = current_mouse_pos[0]; //right, so these are correct?
    let begin_col: i32 = start_mouse_pos[1]; //i guess you're right, but it doesn't change what's going on lol
    let fin_col: i32 = current_mouse_pos[1]; //fair

    let x_component:i32 = (fin_row - begin_row).abs(); //and these
    let y_component:i32 = (fin_col - begin_col).abs();
    // Circle is faster, so do not waste time using this tool if it's a circle
    if x_component == y_component {
        circle_tool(main_window,
        &[begin_row, begin_col],
        &[fin_row, fin_col]);
        return;
    }
    else if y_component == 0 { //that's why these were flipped. they're misnamed
        println!("We entered the statement"); //these will make normal lines now, but only that
        line_tool(main_window,
        &[begin_row + (-2 * (begin_row - fin_row)), begin_col], //does this feel weird to you too? like these may be wrong, but they aren't displaying what they should, right?
        &[begin_row, begin_col], //didn't matter, they should both be the same. since x_comp = 0, both x are same
        true); //i've been messing with both lol
        return; //these should both be straight lines.
    }
    else if x_component == 0 { //oh it'll work fine if you do this, but change ANYTHING, and it goes bonkers
        println!("We entered the other statement");
        line_tool(main_window,
        &[fin_row, begin_col + (-2 * (begin_col - fin_col))],
        &[begin_row, begin_col],
        false); //this should be futureproofed when we decide to do odd ellipses
        return;  
    }
    //passed to draw_ellipse
    let draw_quad_pixels = |main_window: &mut main_window::MainWindow<'_>, begin_row: i32, begin_col: i32, x: i32, y: i32| {
        //mentioned in previous credits's source, but I figured I'd be specific https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/
        //draw_quad_pixels in doc.rs
        main_window.add_to_preview_buffer(begin_row + x, begin_col + y);
        main_window.add_to_preview_buffer(begin_row - x, begin_col + y);
        main_window.add_to_preview_buffer(begin_row + x, begin_col - y);
        main_window.add_to_preview_buffer(begin_row - x, begin_col - y);
    };

    draw_ellipse(main_window,
    draw_quad_pixels,
    &[begin_row, begin_col],
    &[fin_row, fin_col]);
}

fn filled_ellipse_tool(main_window: &mut main_window::MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {
    //docs.rs draw_filled_ellipse_mut, same source as above

    main_window.preview_buffer.clear();

    let begin_row: i32 = start_mouse_pos[0];
    let fin_row: i32 = current_mouse_pos[0];
    let begin_col: i32 = start_mouse_pos[1];
    let fin_col: i32 = current_mouse_pos[1];

    let x_component:i32 = (fin_row - begin_row).abs();
    let y_component:i32 = (fin_col - begin_col).abs();

    //same as above tool, circle will be faster
    if x_component == y_component {
        filled_circle_tool(main_window,
        &[begin_row, begin_col],
        &[fin_row, fin_col]);
        return;
    }
    //will be passed to draw_ellipse to draw line pair when drawing
    let draw_line_pairs = |main_window: &mut main_window::MainWindow<'_>, begin_row: i32, begin_col: i32, x: i32, y: i32| {
        line_tool(
            main_window,
            &[(begin_row - x), (begin_col + y)],
            &[(begin_row + x), (begin_col + y)],
            false );
        line_tool(
            main_window,
            &[(begin_row - x), (begin_col - y)],
            &[(begin_row + x) , (begin_col - y)],
            false );
    };

    draw_ellipse(main_window,
        draw_line_pairs,
        &[begin_row, begin_col],
        &[fin_row, fin_col]);
}

fn text_tool(main_window: &mut main_window::MainWindow<'_>, &prev_gpos: &[i32;2], input: &String) -> [i32;2] {
    let text_vec: Vec<char> = input.chars().collect();
    if prev_gpos[1] >= (main_window.num_of_cols as i32) {
        main_window.window_array[prev_gpos[0] as usize][(main_window.num_of_cols - 1) as usize] = text_vec[0];
    }
    else {
        main_window.window_array[prev_gpos[0] as usize][prev_gpos[1] as usize] = text_vec[0];
    }
    return [prev_gpos[0], std::cmp::min(prev_gpos[1]+1, (main_window.num_of_cols as i32)-1)];
}

fn free_tool(main_window: &mut main_window::MainWindow<'_>, current_mouse_pos: &[i32; 2], prev_gpos: &[i32; 2]){
    if current_mouse_pos != prev_gpos{
        main_window.add_to_preview_buffer(current_mouse_pos[0], current_mouse_pos[1]);
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("failed to init sdl");
    let video_subsystem = sdl_context.video().expect("failed to init video subsytem");
    let clipboard = video_subsystem.clipboard();
    let ttf_context = sdl2::ttf::init().unwrap(); //Maybe add a error message

    let mut main_window = main_window::MainWindow::new(
        &sdl_context,
        &ttf_context,
        &video_subsystem,
        &clipboard,
        1200,
        900,
        60,
        40,
        100,
    );
        
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
                            if y > main_window.gui_height as i32{
                                let gpos = main_window.get_mouse_gpos(x, y);
                                if &current_tool == "f"{
                                    free_tool(&mut main_window, &gpos, &[-1, -1]);
                                }
                                else if &current_tool == "l"{
                                    mstart_pos = gpos;
                                    line_tool(&mut main_window, &gpos, &mstart_pos, true);
                                }
                                else if &current_tool == "r"{
                                    mstart_pos = gpos;
                                    rectangle_tool(&mut main_window, &gpos, &mstart_pos)
                                }
                                else if &current_tool == "s"{
                                    mstart_pos = gpos;
                                    filled_rectangle_tool(&mut main_window, &gpos, &mstart_pos);
                                }
                                else if &current_tool == "o"{
                                    mstart_pos = gpos;
                                    circle_tool(&mut main_window, &gpos, &mstart_pos);
                                }
                                else if &current_tool == "q"{
                                    mstart_pos = gpos;
                                    filled_circle_tool(&mut main_window, &gpos, &mstart_pos);
                                }
                                else if &current_tool == "p"{
                                    mstart_pos = gpos;
                                    rectangle_tool(&mut main_window, &gpos, &mstart_pos)
                                }
                                else if &current_tool == "e"{
                                    mstart_pos = gpos;
                                    ellipse_tool(&mut main_window, &gpos, &mstart_pos);
                                }
                                else if &current_tool == "w"{
                                    mstart_pos = gpos;
                                    filled_ellipse_tool(&mut main_window, &gpos, &mstart_pos);
                                }
                                prev_gpos = gpos;
                            }
                            else{
                                //gui stuff goes here
                            }
                        },
                        _ => {}, //eventually will be replaced with a tool list
                    }
                    render_change = true;
                },
                Event::MouseMotion {mousestate, x, y, ..} => { //this is for holding down button
                    if mousestate.left(){
                        if y > main_window.gui_height as i32{
                            let gpos = main_window.get_mouse_gpos(x, y);
                            if &current_tool == "f"{ //gives these functions the needed parameters
                                free_tool(&mut main_window, &gpos, &prev_gpos);
                            }
                            else if &current_tool == "l"{
                                line_tool(&mut main_window, &gpos, &mstart_pos, true);
                            }
                            else if &current_tool == "r"{
                                rectangle_tool(&mut main_window, &gpos, &mstart_pos)
                            }
                            else if &current_tool == "s"{
                                filled_rectangle_tool(&mut main_window, &gpos, &mstart_pos)
                            }
                            else if &current_tool == "o"{
                                circle_tool(&mut main_window, &gpos, &mstart_pos);
                            }
                            else if &current_tool == "q"{
                                filled_circle_tool(&mut main_window, &gpos, &mstart_pos);
                            }
                            else if &current_tool == "p"{
                                rectangle_tool(&mut main_window, &gpos, &mstart_pos)
                            }
                            else if &current_tool == "e"{
                                ellipse_tool(&mut main_window, &gpos, &mstart_pos);
                            }
                            else if &current_tool == "w"{
                                filled_ellipse_tool(&mut main_window, &gpos, &mstart_pos);
                            }
                            if prev_gpos != gpos{
                                render_change = true;
                            }
                            prev_gpos = gpos;
                        }
                    }
                },
                Event::MouseButtonUp {mouse_btn, x, y, ..} => { //let go
                    match mouse_btn{
                        sdl2::mouse::MouseButton::Left => {
                            if &current_tool == "p"{
                                let gpos = main_window.get_mouse_gpos(x, y);
                                image_conv::convert_image_put_in_window(&mut main_window.window_array, 
                                                                        &gpos, &mstart_pos, 
                                                                        &tool_modifier[0], &tool_modifier[1]
                                ); 
                                main_window.preview_buffer.clear();
                            }
                            else{
                                main_window.write_buffer(current_key);
                            }
                            render_change = true;
                        },
                        _ => {}
                    }
                },
                Event::TextInput {text, ..} => { //keyboard determines keycombo (keybinds)
                    println!("text: {}", text);
                    if &current_tool == "t"{
                        prev_gpos = text_tool(&mut main_window, &prev_gpos, &text); //text mode case
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
                            main_window.copy_to_clipboard();
                        }
                        else if &(text.to_lowercase()) == "m"{
                            keycombo = String::from("m");
                        }
                        else if &(text.to_lowercase()) == "z"{
                            main_window.undo_redo.perform_undo(&mut main_window.window_array);
                            render_change = true;
                        }
                        else if &(text.to_lowercase()) == "y"{
                            main_window.undo_redo.perform_redo(&mut main_window.window_array);
                            render_change = true;
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
                                if prev_gpos[1] == (main_window.num_of_cols as i32) - 1 && //updates our position
                                main_window.window_array[prev_gpos[0] as usize][prev_gpos[1] as usize] != ' '{ //moves to that new position
                                    end_offset = 1;
                                }
                                main_window.window_array[prev_gpos[0] as usize][(std::cmp::max(prev_gpos[1]-1+end_offset, 0)) as usize] = ' ';
                                prev_gpos = [prev_gpos[0], std::cmp::max(prev_gpos[1] - 1 + end_offset, 0)];
                                render_change = true;
                            }
                            Some(sdl2::keyboard::Keycode::UP) => { //directions??? No way, it's 4024
                                prev_gpos = [std::cmp::max(prev_gpos[0]-1, 0), prev_gpos[1]];
                            }
                            Some(sdl2::keyboard::Keycode::DOWN) => {
                                prev_gpos = [std::cmp::min(prev_gpos[0]+1, (main_window.num_of_rows as i32)-1), prev_gpos[1]];
                            }
                            Some(sdl2::keyboard::Keycode::LEFT) => {
                                prev_gpos = [prev_gpos[0], std::cmp::max(prev_gpos[1]-1, 0)];
                            }
                            Some(sdl2::keyboard::Keycode::RIGHT) => {
                                prev_gpos = [prev_gpos[0], std::cmp::min(prev_gpos[1]+1, (main_window.num_of_cols as i32)-1)];
                            }
                            _ => {}
                        }   
                    }
                },
                Event::Window {win_event, ..} =>{
                    match win_event{
                        sdl2::event::WindowEvent::SizeChanged(width, height) => {
                            main_window.window_size_changed(width, height);
                            render_change = true;
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        if render_change{ //render if change
            let pre = std::time::SystemTime::now();
            main_window.render(current_key);
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
