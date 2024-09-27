extern crate sdl2;

use std::collections::linked_list::Iter;

use sdl2::event::Event; // Rust equivalent of C++ using namespace. Last "word" is what you call
use sdl2::pixels::Color; // Like call this with Color::

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,  //main render function
          font: &sdl2::ttf::Font, //our chosen font
          window_array: &Vec<Vec<char>>, //3d array for our window
          clen: i32, rlen: i32){ //our dimenions for the canvas
    
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

    canvas.present(); //actually commit changes to screen!
}

fn get_mouse_gpos(cpos: i32, rpos: i32, clen: i32, rlen: i32) -> [i32; 2]{
    return [rpos / rlen, cpos / clen]; //converts window dimensions to canvas dimensions
}

fn line_tool(preview_buffer: &mut Vec<[i32; 3]>,current_mouse_pos: [i32; 2], start_mouse_pos: [i32; 2]){ //need to implement preview buffer & out of bounds...?
    let mut beginx: i32 = start_mouse_pos[0]; //break up into 1 dimension bc lines are 1-dimensional
    let mut beginy: i32 = start_mouse_pos[1];
    let mut finx: i32 = current_mouse_pos[0];
    let mut finy: i32 = current_mouse_pos[1];
    let mut x_iterator: i32 = 0;
    let mut y_iterator: i32 = 0;
    let mut line_draw: i32 = 0;

    x_iterator = match x_iterator { //make sure only 1 direction (no diagonals rn) & will iterate later
        _ if beginx < finx => 1,
        _ if beginx > finx => -1,
        _ => 0,
    };
    y_iterator = match y_iterator {
        _ if beginy < finy => 1,
        _ if beginy > finy => -1,
        _ => 0,
    };

    match line_draw { //cases for 4 directions & they work!, but will always draw from left to right or top to bottom when iterating thru loop
        _ if (x_iterator == 1) && (y_iterator == 0) => //from left to right
            { for _ in start_mouse_pos[0]..=current_mouse_pos[0]
                {
                window_array[beginx as usize][beginy as usize] = 'a';
                beginx += x_iterator;
                }
            },
        _ if (x_iterator == -1) && (y_iterator = 0) => //from right to left
            { for _ in current_mouse_pos[0]..=start_mouse_pos[0]
                {
                window_array[beginx as usize][beginy as usize] = 'a';
                beginx -= x_iterator;
                }
            },
        _ if (x_iterator == 0) && (y_iterator == 1) => //from top to bottom
            { for _ in start_mouse_pos[1]..=current_mouse_pos[1]
                {
                window_array[beginx as usize][beginy as usize] = 'a';
                beginy += y_iterator;
                }
            },
        _ if (x_iterator == 0) && (y_iterator == -1) => //from bottom to top
            { for _ in current_mouse_pos[1]..=start_mouse_pos[1]
                {
                window_array[beginx as usize][beginy as usize] = 'a';
                beginy -= y_iterator;
                }
            },
        _ if (x_iterator == 0) && (y_iterator == -1) //if both 0, then line is just a single letter
            {
            window_array[beginx as usize][beginy as usize] = 'a'; 
            },
        _ => {} //if some random scenario, nothing
    }
} //commit changes after run


fn rectangle_tool(preview_buffer: &mut Vec<[i32; 3]>, current_mouse_pos: [i32; 2], start_mouse_pos: [i32; 2]){ //need to implement preview buffer

    line_tool(preview_buffer,[start_mouse_pos[0], start_mouse_pos[1]],[current_mouse_pos[0], start_mouse_pos[1]]); //this is usually top assuming
    //top left to bottom right
    line_tool(preview_buffer,[start_mouse_pos[0], start_mouse_pos[1]],[start_mouse_pos[0], current_mouse_pos[1]]); //left

    line_tool(preview_buffer,[start_mouse_pos[0], current_mouse_pos[1]],[current_mouse_pos[0], current_mouse_pos[1]]); //bottom

    line_tool(preview_buffer,[current_mouse_pos[0], start_mouse_pos[1]],[current_mouse_pos[0], current_mouse_pos[1]]); //right

} //change after this is done running


fn main() {
    
    let window_width: u32 = 1200;
    let window_height: u32 = 800;
    let num_of_cols: u32 = 60;
    let num_of_rows: u32 = 40;
    let col_length: i32 = (window_width / num_of_cols) as i32;
    let row_length: i32 = (window_height / num_of_rows) as i32;
    let mut preview_buffer = Vec::new();

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
        


    let mut event_queue = sdl_context.event_pump().expect("failed to init event queue");
    let mut running = true;
    let mut render_change = true;
    while running { //main loop
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
                        _ => {}, //include left-click erase, eventually tool list in match outside
                    }
                    render_change = true;
                },
                Event::MouseMotion {mousestate, x, y, ..} => { //this is for holding down button
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
        if render_change{ //render if change
            render(&mut canvas, &font, &window_array, col_length, row_length);
            render_change = false;
        }
    }
}


