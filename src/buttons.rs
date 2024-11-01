use crate::main_window::MainWindow;

pub fn slice(multiple: f32) -> u32 { //multiplies
    return (multiple * main_window.window_width) as u32;
}
//subbuttons
pub fn subbutton(x: i32) {
    match current_gui_button {
         => println!("Subbutton test"),
        _ => {}
    }
}

else if y > (main_window.gui_height as f32 * 0.5) as i32 { //bottom row of buttons
    match current_gui_button {
        1 => println!("Subbutton test"),
        _ => {}
    }
}

//buttons
else if y <= (main_window.gui_height as f32 * 0.5) as i32 && y >= 0 { //top row of buttons
    if x >= 0 && x < (main_window.window_width as f32 *  0.2) as i32 { //range check
        println!("button 1 pressed");
        current_gui_button = 1; //this whites out, change to create subbuttons,
    }
    else if x >= (main_window.window_width as f32 *  0.2) as i32 && x < (main_window.window_width as f32 *  0.4) as i32 {
        current_gui_button = 2;
        println!("button 2 pressed");
    }
    else if x >= (main_window.window_width as f32 *  0.4) as i32 && x < (main_window.window_width as f32 *  0.6) as i32 {
        current_gui_button = 3;
        println!("button 3 pressed");
    }
    else if x >= (main_window.window_width as f32 *  0.6) as i32 && x < (main_window.window_width as f32 *  0.8) as i32 {
        current_gui_button = 4;
        println!("button 4 pressed");
    }
    else if x >= (main_window.window_width as f32 *  0.8) as i32 && x <= main_window.window_width as i32 {
        current_gui_button = 5;
        println!("button 5 pressed");
    }
}

//render stuff

       //top row of buttons are placed on an offset of 1/5 of the window width (All buttons are 1/5th of window width wide)
       self.canvas.set_draw_color(Color::RGB(50, 100, 150)); //3 navy blue rectangles on top of background
       let _ = self.canvas.fill_rect(sdl2::rect::Rect::new((self.window_width as f32 *  0.2) as i32, 0,
           self.window_width / 5u32, (self.gui_height as f32 * 0.5) as u32)); //these are done differently for readability (decimals and fractions)
       let _ = self.canvas.fill_rect(sdl2::rect::Rect::new((self.window_width as f32 *  0.6) as i32, 0,
           self.window_width / 5u32, (self.gui_height as f32 * 0.5) as u32)); 
   
       if current_gui_button == 0 { //removes subbuttons visually
           self.canvas.set_draw_color(Color::RGB(125, 125, 125)); //color = grey
           let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(0, (self.gui_height as f32 * 0.5) as i32, //subbuttons get covered up
               self.window_width, (self.gui_height as f32 * 0.5) as u32));
       }
       else if current_gui_button == 1 || current_gui_button == 2 { //3 buttons case
           self.canvas.set_draw_color(Color::RGB(50, 100, 150)); //navy background
           let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(0, (self.gui_height as f32 * 0.5) as i32,
                   self.window_width, (self.gui_height as f32 * 0.5) as u32));
           
           self.canvas.set_draw_color(Color::RGB(255, 255, 255)); //middle white button             
           let _ = self.canvas.fill_rect(sdl2::rect::Rect::new((self.window_width as f32 * 1.0/3.0) as i32,
           (self.gui_height as f32 * 0.5) as i32, self.window_width / 3u32, (self.gui_height as f32 * 0.5) as u32));
       }
       else if current_gui_button == 3 || current_gui_button == 4 {
           self.canvas.set_draw_color(Color::RGB(50, 100, 150)); //left navy button
           let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(0, (self.gui_height as f32 * 0.5) as i32,
                   self.window_width / 2u32, (self.gui_height as f32 * 0.5) as u32));
           self.canvas.set_draw_color(Color::RGB(255, 255, 255)); //right white button
               let _ = self.canvas.fill_rect(sdl2::rect::Rect::new((self.window_width as f32 * 0.5) as i32, (self.gui_height as f32 * 0.5) as i32,
               self.window_width / 2u32, (self.gui_height as f32 * 0.5) as u32)); 
       }
       else if current_gui_button == 5 {
           self.canvas.set_draw_color(Color::RGB(50, 100, 150)); //1 button case
           let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(0, (self.gui_height as f32 * 0.5) as i32,
           self.window_width, (self.gui_height as f32 * 0.5) as u32)); 
       }
   
       let button_string = "File";
       let input_string: &str = &button_string;
       let gui_button_col_pos_iterator:f32 = 0.0;
       let gui_button_row_pos_iterator:f32 = 0.0;
   
       let mut write_button = |input_string: &str| {
           let font_render = self.font.render(&input_string); //create a render of the given string
           let font_surface = font_render.blended_wrapped(Color::RGB(255, 255, 255), 0).unwrap(); //create a surface out of that render
           let canvas_texture = self.canvas.texture_creator(); //generate a blank canvas from the canvas 
           let texture = canvas_texture.create_texture_from_surface(font_surface).unwrap(); //copy the font surface onto that texture
           let _ = self.canvas.copy(
               &texture,
               None, //part of texture we want... all of it 
               sdl2::rect::Rect::new(0, (self.gui_height as f32 * (1.0/8.0)) as i32,
                                     self.window_width / 5, self.gui_height / 4) //first two is where, second is how big
           ).expect("failed copying texture to canvas"); //display that texture to the canvas
       };
       //test text
       write_button(input_string);