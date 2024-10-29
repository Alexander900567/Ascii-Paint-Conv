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

    let mut horizontal_slope = fin_row - begin_row;
    let mut vertical_slope = fin_col - begin_col;
    let mut row_iter = 0;
    let mut col_iter = 0;

    if horizontal_slope != 0 {
        row_iter = horizontal_slope / horizontal_slope.abs(); 
    }
    if vertical_slope != 0 {
        col_iter = vertical_slope / vertical_slope.abs(); 
    }

    horizontal_slope = horizontal_slope.abs();
    vertical_slope = vertical_slope.abs();

    let long_slope;
    let short_slope;
    let row_length_is_long;
    if horizontal_slope > vertical_slope {
        long_slope = horizontal_slope;
        short_slope = vertical_slope + 1;
        row_length_is_long = true;
    }
    else {
        long_slope = vertical_slope;
        short_slope = horizontal_slope + 1;
        row_length_is_long = false;
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
            if row_length_is_long {
                begin_row += row_iter;   
            }
            else {
                begin_col += col_iter;
            }
        }
        if !row_length_is_long {
            begin_row += row_iter;   
        }
        else{
            begin_col += col_iter;
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
    
    let leftmost_row:i32;
    let rightmost_row:i32;

    if begin_row <= fin_row { //right quadrants case
        leftmost_row = begin_row;
        rightmost_row = fin_row;
    }
    else { //left quadrants case
        leftmost_row = fin_row;
        rightmost_row = begin_row;
    }

    for row_num in leftmost_row..=rightmost_row { //iterates vertical lines
        line_tool(main_window,
        &[row_num, begin_col], //further iterates those lines horizontally (left to right or right to left)
        &[row_num, fin_col],
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

    let row_dif:i32 = fin_row - begin_row;
    let col_dif:i32 = fin_col - begin_col;
    let r:i32;
    let diagonal_r:f32 = f32::sqrt((row_dif as f32 * row_dif as f32) + (col_dif as f32 * col_dif as f32)); //pythag h
    //theory: given r = 10
    /* diagonal_r = real hypotenuse = 10*sqrt(2) = r*ratio
    r (radius of circle)= diagonal_r / ratio
    ratio = hypotenuse of a triangle with sides divided by r with same angle theta = h = (o/r)/sin(theta) 
    theta = sin^-1(o/diagonal_r) */
    match(row_dif, col_dif) {
        (row_num,col_num) if row_num != 0 && col_num != 0 => { //non-cardinal case
        let o:i32 = col_dif.abs(); //to keep scalar factor positive (since we're about to use sin)
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

    let mut row_num:i32 = 0i32;
    let mut col_num:i32 = r; //(row_num,col_num) = (0,r)
    let mut p:i32 = 1 - r;

    while row_num <= col_num {
        main_window.add_to_preview_buffer(begin_row + row_num, begin_col + col_num);
        main_window.add_to_preview_buffer(begin_row + col_num, begin_col + row_num);
        main_window.add_to_preview_buffer(begin_row - col_num, begin_col + row_num);
        main_window.add_to_preview_buffer(begin_row - row_num, begin_col + col_num);
        main_window.add_to_preview_buffer(begin_row - row_num, begin_col - col_num);
        main_window.add_to_preview_buffer(begin_row - col_num, begin_col - row_num);
        main_window.add_to_preview_buffer(begin_row + col_num, begin_col - row_num);
        main_window.add_to_preview_buffer(begin_row + row_num, begin_col - col_num);

        row_num += 1; //all 4 regions
        if p < 0 {
            p += 2 * row_num + 1
        }
        else {
            col_num -= 1;
            p += 2 * (row_num - col_num) + 1;
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

    let row_dif:i32 = fin_row - begin_row;
    let col_dif:i32 = fin_col - begin_col;
    let r:i32;
    let diagonal_r:f32 = f32::sqrt((row_dif as f32 * row_dif as f32) + (col_dif as f32 * col_dif as f32));

    match(row_dif, col_dif) {
        (row_num,col_num) if row_num != 0 && col_num != 0 => { //non-cardinal case
        let o:i32 = col_dif.abs(); //to keep scalar factor positive (since we're about to use sin)
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

    let mut row_num = 0i32;
    let mut col_num = r;
    let mut p = 1 - r; //haven't assigned r, assign later

    while row_num <= col_num {

        line_tool(main_window,
        &[(begin_row + row_num), (begin_col + col_num)], 
        &[(begin_row - row_num), (begin_col + col_num)],
        false);

        line_tool(main_window,
        &[(begin_row + col_num), (begin_col + row_num)], 
        &[(begin_row - col_num), (begin_col + row_num)],
        false);

        line_tool(main_window,
        &[(begin_row + row_num), (begin_col - col_num)], 
        &[(begin_row - row_num), (begin_col - col_num)],
        false);

        line_tool(main_window,
        &[(begin_row + col_num), (begin_col - row_num)], 
        &[(begin_row - col_num), (begin_col - row_num)],
        false);

        row_num += 1;
        if p < 0 {
            p += 2 * row_num + 1;
        }
        else {
            col_num -= 1;
            p += 2 * (row_num - col_num) + 1;
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

    let row_dif:i32 = (fin_row - begin_row).abs();
    let col_dif:i32 = (fin_col - begin_col).abs();
    let row_diff_squared: f32 = (row_dif * row_dif) as f32;
    let col_diff_squared: f32 = (col_dif * col_dif) as f32;

    let mut row_num: i32 = 0;
    let mut col_num: i32 = col_dif;

    let mut p_row:f32 = 0.0;
    let mut p_col:f32 = 2.0 * row_diff_squared * col_num as f32;

    render_func(main_window, begin_row, begin_col, row_num, col_num);

    //Top and bottom
    let mut p:f32 = col_diff_squared - (row_diff_squared * col_dif as f32) + (0.25f32 * row_diff_squared);
    while p_row <= p_col {
        row_num += 1;
        p_row += 2.0 * col_diff_squared;
        if p < 0.0 {
            p += col_diff_squared + p_row;
        }
        else {
            col_num -= 1;
            p_col += -2.0 * row_diff_squared;
            p += col_diff_squared + p_row - p_col;
        }
        render_func(main_window, begin_row, begin_col, row_num, col_num);
    }

    //Left and right
    p = (col_diff_squared * ((row_num as f32 + 0.5).powi(2))) + (row_diff_squared * (col_num - 1).pow(2) as f32) - (row_diff_squared * col_diff_squared);
    while col_num >= 0 {
        col_num -= 1;
        p_col += -2.0 * row_diff_squared;
        if p > 0.0 {
            p += row_diff_squared - p_col;
        }
        else {
            row_num += 1;
            p_row += (2.0 * col_diff_squared);
            p += row_diff_squared - p_col + p_row;
        }
        render_func(main_window, begin_row, begin_col, row_num, col_num);
    }
}

fn ellipse_tool(main_window: &mut main_window::MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {
    //docs.rs draw_hollow_ellipse_mut

    main_window.preview_buffer.clear();

    let begin_row: i32 = start_mouse_pos[0]; //noted, will rename, nvm now, but only this section for now [0] is row, right?
    let fin_row: i32 = current_mouse_pos[0]; //right, so these are correct?
    let begin_col: i32 = start_mouse_pos[1]; //i guess you're right, but it doesn't change what's going on lol
    let fin_col: i32 = current_mouse_pos[1]; //fair

    let row_dif:i32 = (fin_row - begin_row).abs(); //and these
    let col_dif:i32 = (fin_col - begin_col).abs();
    // Circle is faster, so do not waste time using this tool if it's a circle
    if row_dif == col_dif {
        circle_tool(main_window,
        &[begin_row, begin_col],
        &[fin_row, fin_col]);
        return;
    }
    else if col_dif == 0 { //Straight line is faster
    line_tool(main_window,
    &[begin_row + (-2 * (begin_row - fin_row)), begin_col],
    &[begin_row, begin_col],
    true);
    return; //only do straight line, no ellipse at all
    }
    else if row_dif == 0 { 
    line_tool(main_window,
    &[fin_row, begin_col + (-2 * (begin_col - fin_col))],
    &[begin_row, begin_col],
    true); 
    return;  
    }
    //passed to draw_ellipse
    let draw_quad_pixels = |main_window: &mut main_window::MainWindow<'_>, begin_row: i32, begin_col: i32, row_num: i32, col_num: i32| {
        //mentioned in previous credits's source, but I figured I'd be specific https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/
        //draw_quad_pixels in doc.rs
        main_window.add_to_preview_buffer(begin_row + row_num, begin_col + col_num);
        main_window.add_to_preview_buffer(begin_row - row_num, begin_col + col_num);
        main_window.add_to_preview_buffer(begin_row + row_num, begin_col - col_num);
        main_window.add_to_preview_buffer(begin_row - row_num, begin_col - col_num);
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

    let row_dif:i32 = (fin_row - begin_row).abs();
    let col_dif:i32 = (fin_col - begin_col).abs();

    //same as above tool, circle will be faster
    if row_dif == col_dif {
        filled_circle_tool(main_window,
        &[begin_row, begin_col],
        &[fin_row, fin_col]);
        return;
    }
    //will be passed to draw_ellipse to draw line pair when drawing
    let draw_line_pairs = |main_window: &mut main_window::MainWindow<'_>, begin_row: i32, begin_col: i32, row_num: i32, col_num: i32| {
        line_tool(
            main_window,
            &[(begin_row - row_num), (begin_col + col_num)],
            &[(begin_row + row_num), (begin_col + col_num)],
            false );
        line_tool(
            main_window,
            &[(begin_row - row_num), (begin_col - col_num)],
            &[(begin_row + row_num) , (begin_col - col_num)],
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
