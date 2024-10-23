use sdl2::pixels::Color;

pub struct MainWindow<'a> {
        
    sdl_context: &'a sdl2::Sdl,
    ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    video_subsystem: &'a sdl2::VideoSubsystem,
    clipboard: &'a sdl2::clipboard::ClipboardUtil,
    canvas: sdl2::render::WindowCanvas,
    font: sdl2::ttf::Font<'a, 'a>,

    window_width: u32,
    window_height: u32, 
    pub num_of_cols: u32,
    pub num_of_rows: u32, 
    col_length: i32,
    row_length: i32,
    pub preview_buffer: Vec<[i32;2]>,
    pub window_array: Vec<Vec<char>>, 
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

        let start_window = video_subsystem.window("ascii", window_width, window_height) //builds and names window
            .position_centered()
            .build()
            .expect("failed to build window");

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
            col_length: (window_width / num_of_cols) as i32,
            row_length: (window_height / num_of_rows) as i32,
            preview_buffer: Vec::new(),
            window_array: start_window_array,
        }
    }

    pub fn render(&mut self, current_key: char){
        let mut render_array = self.window_array.clone();
        
        for buffer_item in &self.preview_buffer{ 
            render_array[buffer_item[0] as usize][buffer_item[1] as usize] = current_key;
        }

        self.canvas.set_draw_color(Color::RGB(0, 0, 0)); //set canvas to black
        self.canvas.clear(); //clears frame allows new one
        
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
            sdl2::rect::Rect::new(0, 0, 1200, 800), //first two is where, second is how big
        ).expect("failed copying texture to canvas"); //display that texture to the canvas


        self.canvas.present(); //actually commit changes to screen!
    }

    pub fn write_buffer(&mut self, current_char: char) {
        for buffer_item in &(self.preview_buffer){
            self.window_array[buffer_item[0] as usize][buffer_item[1] as usize] = current_char;
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
        let mut rgpos: i32 = rpos / self.row_length; //row global position, row position, row length
        let mut cgpos: i32 = cpos / self.col_length; // same but column
        let rnumi = self.num_of_rows as i32;
        let cnumi = self.num_of_cols as i32;
    
        if rgpos < 0 {rgpos = 0;} //sets 0 as left bound
        else if rgpos >= rnumi {rgpos = rnumi - 1;} //right bound
        if cgpos < 0 {cgpos = 0;} //upper bound
        else if cgpos >= cnumi {cgpos = cnumi - 1;} //lower bound
    
        return [rgpos, cgpos]; //converts window dimensions to canvas dimensions
    }

    pub fn add_to_preview_buffer(&mut self, rpos: i32, cpos: i32){
        //don't add to preview buffer if out of bounds
        let rnumi = self.num_of_rows as i32;
        let cnumi = self.num_of_cols as i32;
        if (rpos < 0) || (rpos >= rnumi) || (cpos < 0) || (cpos >= cnumi){
            return;
        }
        self.preview_buffer.push([rpos, cpos]);
    }
}








