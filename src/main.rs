extern crate sdl2;
extern crate image;
extern crate rayon;
mod main_window;
mod tools;
mod image_conv;
mod undo_redo;
mod gui;
mod save_load;
mod rectangle_selector;

use sdl2::event::Event; // Rust equivalent of C++ using namespace. Last "word" is what you call
use image::{DynamicImage, GenericImage};

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


    let mut gui_bar = gui::Gui::new(main_window.gui_height, main_window.window_width, 8, 20);

    let mut toolbox = tools::Toolbox::new();
        
    video_subsystem.text_input().start();

    /////////
    let mut times = Vec::new();
    ////

    let mut path_to_save_file = String::from("");
    let mut event_queue = sdl_context.event_pump().expect("failed to init event queue");
    let mut running = true;
    //decide whether or not to render a new frame (only render when something has a changed)
    let mut render_change = true;
    //holds on to the previous loops' gpos so a render doesn't get called if the mouse hasn't moved grid position
    let mut keycombo = String::new(); //will hold our key commands
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
                                let _ = toolbox.draw_tool(&mut main_window, true, x, y);
                            }
                            else{
                                gui_bar.handle_gui_click(x, y, &mut main_window, &mut toolbox);
                            }
                        },
                        _ => {}, //eventually will be replaced with a tool list
                    }
                    render_change = true;
                },
                Event::MouseMotion {mousestate, x, y, ..} => { //this is for holding down button
                    if mousestate.left(){
                        if y > main_window.gui_height as i32{
                            render_change = toolbox.draw_tool(&mut main_window, false, x, y);
                        }
                    }
                },
                Event::MouseButtonUp {mouse_btn, x, y, ..} => { //let go
                    match mouse_btn{
                        sdl2::mouse::MouseButton::Left => {
                            if y > main_window.gui_height as i32{
                                if &toolbox.current_tool == "p"{
                                    main_window.preview_buffer.clear();
                                    let gpos = main_window.get_mouse_gpos(x, y);
                                    image_conv::convert_image_put_in_window(&mut main_window, 
                                                                            &gpos, &toolbox.mstart_gpos, 
                                                                            &toolbox.ascii_type, toolbox.ascii_edges
                                    ); 
                                }
                                else if &toolbox.current_tool == "a"{
                                    toolbox.rect_sel_tool.on_mouse_up(&mut main_window);
                                }
                            }

                            if (Vec::from(["a"]).iter().any(|x| x != &toolbox.current_tool) && 
                                main_window.preview_buffer.len() > 0){
                                main_window.write_buffer();
                            }
                            render_change = true;
                        },
                        _ => {}
                    }
                },
                Event::TextInput {text, ..} => { //keyboard determines keycombo (keybinds)
                    println!("text: {}", text);
                    if &toolbox.current_tool == "t"{
                        toolbox.text(&mut main_window, &text, "");
                        render_change = true;
                    }
                    else if keycombo.len() > 0{
                        if &keycombo == "i"{
                            let text_vec: Vec<char> = text.chars().collect();
                            toolbox.current_key = text_vec[0];
                        }
                        else if &keycombo == "c"{
                            toolbox.change_tool(&mut main_window, &mut gui_bar, &text.to_lowercase());
                        }
                        else if &keycombo == "m"{
                            toolbox.modify_tool(&mut main_window, &mut gui_bar, &text.to_lowercase());
                        }
                        keycombo = String::from("");
                        render_change = true;
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
                        else if &(text.to_lowercase()) == "s"{
                            path_to_save_file = save_load::save_canvas(&main_window, &path_to_save_file);
                        }
                        else if &(text.to_lowercase()) == "l"{
                            path_to_save_file = save_load::load_canvas(&mut main_window);
                            render_change = true;
                        }
                    }
                },   
                Event::KeyUp {keycode, ..} =>{
                    if &toolbox.current_tool == "t"{
                        match keycode {
                            Some(sdl2::keyboard::Keycode::ESCAPE) =>{ //leave text mode
                                gui_bar.handle_click(0, &mut main_window, &mut toolbox);
                            }
                            Some(sdl2::keyboard::Keycode::BACKSPACE) => {
                                toolbox.text(&mut main_window, &String::from(""), "backspace");
                            }
                            Some(sdl2::keyboard::Keycode::UP) => { //directions??? No way, it's 4024
                                toolbox.text(&mut main_window, &String::from(""), "up");
                            }
                            Some(sdl2::keyboard::Keycode::DOWN) => {
                                toolbox.text(&mut main_window, &String::from(""), "down");
                            }
                            Some(sdl2::keyboard::Keycode::LEFT) => {
                                toolbox.text(&mut main_window, &String::from(""), "left");
                            }
                            Some(sdl2::keyboard::Keycode::RIGHT) => {
                                toolbox.text(&mut main_window, &String::from(""), "right");
                            }
                            _ => {}
                        }   
                        render_change = true;
                    }
                    if &toolbox.current_tool == "a"{
                        match keycode {
                            Some(sdl2::keyboard::Keycode::ESCAPE) =>{
                                toolbox.rect_sel_tool.reset_box(&mut main_window);
                                render_change = true;
                            },
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
                        sdl2::event::WindowEvent::Exposed => {
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
            main_window.render(&gui_bar, &toolbox);
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
