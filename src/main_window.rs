use sdl2::pixels::Color;
use crate::undo_redo;
use crate::gui;
use crate::tools;

use crate::sdl2::image::LoadTexture;
use sdl2::rect::Rect; //may be obselete

pub struct MainWindow<'a> {
        
    sdl_context: &'a sdl2::Sdl,
    ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    video_subsystem: &'a sdl2::VideoSubsystem,
    clipboard: &'a sdl2::clipboard::ClipboardUtil,
    pub canvas: sdl2::render::WindowCanvas,
    font: sdl2::ttf::Font<'a, 'a>,

    pub window_width: u32,
    window_height: u32, 
    pub num_of_cols: u32,
    pub num_of_rows: u32, 
    col_length: f32,
    row_length: f32,
    pub gui_height: u32,
    pub preview_buffer: Vec<(i32, i32, char)>,
    pub window_array: Vec<Vec<char>>, 
    pub undo_redo: undo_redo::UndoRedo, 
}

impl MainWindow<'_>{
    pub fn new<'a>(
            sdl_context: &'a sdl2::Sdl,
            ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
            video_subsystem: &'a sdl2::VideoSubsystem,
            clipboard: &'a sdl2::clipboard::ClipboardUtil,
            window_width: u32,
            window_height: u32, 
            num_of_cols: u32,
            num_of_rows: u32, 
            gui_height: u32,
            ) -> MainWindow<'a>{



        let mut start_window_array = Vec::new();
        for _ in 0..num_of_rows{
            let mut a_row = Vec::new(); 
            for _ in 0..num_of_cols{
                a_row.push(' '); //populate with spaces
            }
            start_window_array.push(a_row);
        }

        let start_font = ttf_context.load_font("./NotoSansMono-Regular.ttf", 16).unwrap();

        let mut start_window = video_subsystem.window("ascii", window_width, window_height) //builds and names window
            .position_centered()
            .build()
            .expect("failed to build window");

        start_window.set_resizable(true);
        //set a min size for the window; doesn't really work on linux for some reason
        let _ = start_window.set_minimum_size(500, gui_height + 100).unwrap();

        let start_canvas = start_window.into_canvas() //builds canvas
            .present_vsync()
            .build()
            .expect("failed to build canvas");



        MainWindow{
            sdl_context: sdl_context,
            ttf_context: ttf_context,
            video_subsystem: video_subsystem,
            clipboard: clipboard,
            canvas: start_canvas,
            font: start_font,
            window_width: window_width,
            window_height: window_height,
            num_of_cols: num_of_cols,
            num_of_rows: num_of_rows,
            col_length: window_width as f32 / num_of_cols as f32,
            row_length: (window_height as f32 - gui_height as f32) / num_of_rows as f32,
            gui_height: gui_height,
            preview_buffer: Vec::new(),
            window_array: start_window_array,
            undo_redo: undo_redo::UndoRedo::new(),
        }
    }

    //render_functions

    pub fn render(&mut self, gui: &gui::Gui, toolbox: &tools::Toolbox){
        
        self.render_grid(toolbox);

        self.render_gui(gui);

        self.canvas.present(); //actually commit changes to screen!
    }

    fn render_gui(&mut self, gui: &gui::Gui){

        self.canvas.set_draw_color(Color::RGB(135, 206, 251)); //set draw color to light blue
        let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(0, 0,
                              self.window_width, self.gui_height)); //first two is where, second is how big, creates rectangle using draw color
        self.canvas.set_draw_color(Color::RGB(90, 90, 90)); //set draw color to grey
        
        let texture_creator = self.canvas.texture_creator();

        let pressed_texture = texture_creator.load_texture("./Assets/PNGs/1x1_button_disabled.png").unwrap();
        let nonpressed_texture = texture_creator.load_texture("./Assets/PNGs/1x1_button_enabled.png").unwrap();

        for button in gui.buttons.values(){
            if button.visible{ //render button
                let top_col = (button.top_left.1 as f32 * gui.col_size) as i32;
                let top_row = (button.top_left.0 as f32 * gui.row_size) as i32;
                let bot_col = ((button.bottom_right.1 - button.top_left.1 + 1) as f32 * gui.col_size) as u32;
                let bot_row = ((button.bottom_right.0 - button.top_left.0 + 1) as f32 * gui.row_size) as u32;
                
                //display button on/off 
                if button.is_pressed == 1 { 
                    let _ = self.canvas.copy(
                        &pressed_texture,
                        None, //part of texture we want... all of it 
                        sdl2::rect::Rect::new(top_col, top_row, bot_col, bot_row));

                }
                else { //TODO: make case for one-shot buttons.
                    let _ = self.canvas.copy(
                        &nonpressed_texture,
                        None, //part of texture we want... all of it 
                        sdl2::rect::Rect::new(top_col, top_row, bot_col, bot_row));
                }

                let icon_texture = texture_creator.load_texture(&button.label_path).unwrap();
                let _ = self.canvas.copy(
                    &icon_texture,
                    None, //part of texture we want... all of it 
                    sdl2::rect::Rect::new(top_col, top_row, bot_col, bot_row));
            }
        }
    }

    pub fn render_grid(&mut self, toolbox: &tools::Toolbox){

        let mut render_array = self.window_array.clone();
        
        for buffer_item in &self.preview_buffer{ 
            render_array[buffer_item.0 as usize][buffer_item.1 as usize] = buffer_item.2;
        }

        self.canvas.set_draw_color(Color::RGB(0, 0, 0)); //set canvas to black
        //self.canvas.clear(); //clears frame allows new one
        let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(0, self.gui_height as i32,
                              self.window_width, self.window_height - self.gui_height)); //first two is where, second is how big
        
        let mut array_string = String::new(); //makes our grid a string, so we can write, copy, etc.
        for x in &render_array{
            for grid_char in x{
                array_string.push(*grid_char);
            }
            array_string.push('\n');
        }

        let font_render = self.font.render(&array_string); //create a render of the given string
        let font_surface = font_render.blended_wrapped(Color::RGB(255, 255, 255), 0).unwrap(); //create a surface out of that render
        let canvas_texture = self.canvas.texture_creator(); //generate a blank canvas from the canvas 
        let texture = canvas_texture.create_texture_from_surface(font_surface).unwrap(); //copy the font surface onto that texture
        let _ = self.canvas.copy(
            &texture,
            None, //part of texture we want... all of it 
            sdl2::rect::Rect::new(0, self.gui_height as i32,
                                  self.window_width, self.window_height - self.gui_height) //first two is where, second is how big
        ).expect("failed copying texture to canvas"); //display that texture to the canvas

        if toolbox.current_tool == "t"{
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            let _ = self.canvas.fill_rect(Rect::new(
                    (toolbox.prev_gpos[1] as f32 * self.col_length).ceil() as i32, 
                    (toolbox.prev_gpos[0] as f32 * self.row_length).ceil() as i32 + self.gui_height as i32,
                    self.col_length as u32, self.row_length as u32)); 
        }
        if toolbox.current_tool == "a"{
            if toolbox.rect_sel_tool.top_left.0 != -1{
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                let _ = self.canvas.draw_rect(Rect::new(
                        (toolbox.rect_sel_tool.top_left.1 as f32 * self.col_length).ceil() as i32, 
                        (toolbox.rect_sel_tool.top_left.0 as f32 * self.row_length).ceil() as i32 + self.gui_height as i32,
                        (toolbox.rect_sel_tool.size.1 as f32 * self.col_length).floor() as u32, 
                        (toolbox.rect_sel_tool.size.0 as f32 * self.row_length).floor() as u32));
            }
        }
    }

    pub fn window_size_changed(&mut self, new_width: i32, new_height: i32) {
        if new_height < self.gui_height as i32{ //emergancy don't crash the program check
            let minimum_height = self.canvas.window().minimum_size().1;
            let _ = self.canvas.window_mut().set_size(new_width as u32, minimum_height).unwrap();
            self.window_height = minimum_height;
        }
        else {self.window_height = new_height as u32;}
        self.window_width = new_width as u32;

        self.col_length = self.window_width as f32 / self.num_of_cols as f32;
        self.row_length = (self.window_height as f32 - self.gui_height as f32) / self.num_of_rows as f32;
    }

    pub fn row_count_change(&mut self, new_row_count: i32){
        if new_row_count < 1{
            return;
        }

        let row_diff: i32 = self.num_of_rows as i32 - new_row_count;
        if row_diff < 0{
            let mut empty_row = Vec::new();
            for _ in 0..self.num_of_cols{
                empty_row.push(' ');
            }
            for _ in 0..row_diff.abs(){
                self.window_array.push(empty_row.clone());
            }
        }
        else if row_diff > 0{
            for _ in 0..row_diff.abs(){
                let _ = self.window_array.pop();
            }
        }
        self.num_of_rows = new_row_count as u32;    

        self.col_length = self.window_width as f32 / self.num_of_cols as f32;
        self.row_length = (self.window_height as f32 - self.gui_height as f32) / self.num_of_rows as f32;
    }

    pub fn col_count_change(&mut self, new_col_count: i32){
        if new_col_count < 1{
            return;
        }

        let col_diff: i32 = self.num_of_cols as i32 - new_col_count;
        if col_diff < 0{
            for r in 0..self.window_array.len(){
                for _ in 0..col_diff.abs(){
                    self.window_array[r].push(' ');
                }
            }
        }
        else if col_diff > 0{
            for r in 0..self.window_array.len(){
                for _ in 0..col_diff.abs(){
                    self.window_array[r].pop();
                }
            }
        }
        self.num_of_cols = new_col_count as u32;

        self.col_length = self.window_width as f32 / self.num_of_cols as f32;
        self.row_length = (self.window_height as f32 - self.gui_height as f32) / self.num_of_rows as f32;
    }
    //grid functions

    pub fn write_buffer(&mut self, write_to_undo: bool) {
        if write_to_undo{
            self.undo_redo.add_to_undo(&self.preview_buffer, &self.window_array);
        }
        self.undo_redo.redo_buffer.clear();
        for buffer_item in &(self.preview_buffer){
            self.window_array[buffer_item.0 as usize][buffer_item.1 as usize] = buffer_item.2;
        }
        self.preview_buffer.clear();
    }

    pub fn copy_to_clipboard(&self) {
        let mut array_string = String::new();
        for x in &(self.window_array) {
            for grid_char in x{
                array_string.push(*grid_char);
            }
            array_string.push('\n');
        }
        let _ = self.clipboard.set_clipboard_text(&array_string).expect("Failed to copy to clipboard");
    }

    pub fn get_mouse_gpos(&self, cpos: i32, rpos: i32) -> [i32; 2] {
        let mut rgpos: i32 = ((rpos as f32 - self.gui_height as f32) / self.row_length) as i32; 
        let mut cgpos: i32 = (cpos as f32 / self.col_length) as i32;
        let rnumi = self.num_of_rows as i32;
        let cnumi = self.num_of_cols as i32;
    
        if rgpos < 0 {rgpos = 0;} //sets 0 as left bound
        else if rgpos >= rnumi {rgpos = rnumi - 1;} //right bound
        if cgpos < 0 {cgpos = 0;} //upper bound
        else if cgpos >= cnumi {cgpos = cnumi - 1;} //lower bound
    
        return [rgpos, cgpos]; //converts window dimensions to canvas dimensions
    }

    pub fn add_to_preview_buffer(&mut self, rpos: i32, cpos: i32, character: char){
        //don't add to preview buffer if out of bounds
        let rnumi = self.num_of_rows as i32;
        let cnumi = self.num_of_cols as i32;
        if (rpos < 0) || (rpos >= rnumi) || (cpos < 0) || (cpos >= cnumi){
            return;
        }
        self.preview_buffer.push((rpos, cpos, character));
    }

    //gui functions
}






