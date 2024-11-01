extern crate sdl2;
extern crate image;
extern crate rayon;
mod main_window;
mod tools;
mod image_conv;
mod undo_redo;
//mod buttons;

use std::thread::current;

use sdl2::event::Event; // Rust equivalent of C++ using namespace. Last "word" is what you call

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
    let mut _current_gui_button = 0;
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
                                    tools::free(&mut main_window, &gpos, &[-1, -1]);
                                }
                                else if &current_tool == "l"{
                                    mstart_pos = gpos;
                                    tools::line(&mut main_window, &gpos, &mstart_pos, true);
                                }
                                else if &current_tool == "r"{
                                    mstart_pos = gpos;
                                    tools::rectangle(&mut main_window, &gpos, &mstart_pos)
                                }
                                else if &current_tool == "s"{
                                    mstart_pos = gpos;
                                    tools::filled_rectangle(&mut main_window, &gpos, &mstart_pos);
                                }
                                else if &current_tool == "o"{
                                    mstart_pos = gpos;
                                    tools::circle(&mut main_window, &gpos, &mstart_pos);
                                }
                                else if &current_tool == "q"{
                                    mstart_pos = gpos;
                                    tools::filled_circle(&mut main_window, &gpos, &mstart_pos);
                                }
                                else if &current_tool == "p"{
                                    mstart_pos = gpos;
                                    tools::rectangle(&mut main_window, &gpos, &mstart_pos)
                                }
                                else if &current_tool == "e"{
                                    mstart_pos = gpos;
                                    tools::ellipse(&mut main_window, &gpos, &mstart_pos);
                                }
                                else if &current_tool == "w"{
                                    mstart_pos = gpos;
                                    tools::filled_ellipse(&mut main_window, &gpos, &mstart_pos);
                                }
                                prev_gpos = gpos;
                            }
                            else {}
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
                                tools::free(&mut main_window, &gpos, &prev_gpos);
                            }
                            else if &current_tool == "l"{
                                tools::line(&mut main_window, &gpos, &mstart_pos, true);
                            }
                            else if &current_tool == "r"{
                                tools::rectangle(&mut main_window, &gpos, &mstart_pos)
                            }
                            else if &current_tool == "s"{
                                tools::filled_rectangle(&mut main_window, &gpos, &mstart_pos)
                            }
                            else if &current_tool == "o"{
                                tools::circle(&mut main_window, &gpos, &mstart_pos);
                            }
                            else if &current_tool == "q"{
                                tools::filled_circle(&mut main_window, &gpos, &mstart_pos);
                            }
                            else if &current_tool == "p"{
                                tools::rectangle(&mut main_window, &gpos, &mstart_pos)
                            }
                            else if &current_tool == "e"{
                                tools::ellipse(&mut main_window, &gpos, &mstart_pos);
                            }
                            else if &current_tool == "w"{
                                tools::filled_ellipse(&mut main_window, &gpos, &mstart_pos);
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
                        prev_gpos = tools::text(&mut main_window, &prev_gpos, &text); //text mode case
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
                        if &(text.to_lowercase()) == "i"{ //will start key select
                            keycombo = String::from("i");
                        }
                        else if &(text.to_lowercase()) == "c"{ //these keys just need to be pressed (no combo)
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
            main_window.render(current_key); //, current_gui_button);
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
