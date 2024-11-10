use crate::main_window::MainWindow;
use crate::gui::Gui;
use std::collections::HashMap;

pub struct Toolbox {
    

    pub current_key: char, 
    pub current_tool: String,
    pub mstart_gpos: [i32; 2],
    pub prev_gpos: [i32; 2],
    pub filled: bool,
    pub ascii_type: String,
    pub ascii_edges: bool,
    tool_letter_to_button_id: HashMap<String, i32>,
    mod_letter_to_button_id: HashMap<String, i32>,
}

impl Toolbox{
    pub fn new() -> Toolbox{
        Toolbox{
            current_key: 'a',
            current_tool: String::from("f"),
            mstart_gpos: [0, 0],
            prev_gpos: [0, 0],
            filled: false,
            ascii_type: String::from("4"),
            ascii_edges: false,
            tool_letter_to_button_id: HashMap::from([(String::from("f"), 0), (String::from("l"), 2), 
                                                    (String::from("r"), 3), (String::from("t"), 5), 
                                                    (String::from("p"), 6), (String::from("o"), 4)]),
            mod_letter_to_button_id: HashMap::from([(String::from("f"), 1), (String::from("e"), 14), 
                                                   (String::from("1"), 10), (String::from("2"), 11), 
                                                   (String::from("3"), 12), (String::from("4"), 13)]),
        }
    }

    pub fn change_tool(&mut self, main_window: &mut MainWindow<'_>, gui_bar: &mut Gui, text: &str){
        let button_id = *self.tool_letter_to_button_id.get(text).unwrap_or(&-1);
        gui_bar.handle_click(button_id, main_window, self);
    }

    pub fn modify_tool(&mut self, main_window: &mut MainWindow<'_>, gui_bar: &mut Gui, text: &str){
        let button_id = *self.mod_letter_to_button_id.get(text).unwrap_or(&-1);
        gui_bar.handle_click(button_id, main_window, self);
    }

    pub fn draw_tool(&mut self, main_window: &mut MainWindow<'_>, click_down: bool, x: i32, y: i32) -> bool{
        let gpos = main_window.get_mouse_gpos(x, y);
        if click_down{
            self.mstart_gpos = gpos;
        }

        if &self.current_tool == "f"{
            self.free(main_window, &gpos, &self.prev_gpos);
        }
        else if &self.current_tool == "l"{
            self.line(main_window, &gpos, &self.mstart_gpos, true);
        }
        else if &self.current_tool == "r"{
            if !self.filled{
                self.rectangle(main_window, &gpos, &self.mstart_gpos)
            }
            else{
                self.filled_rectangle(main_window, &gpos, &self.mstart_gpos);
            }
        }
        else if &self.current_tool == "o"{
            if !self.filled{
                self.circle(main_window, &gpos, &self.mstart_gpos);
            }
            else{
                self.filled_circle(main_window, &gpos, &self.mstart_gpos);
            }
        }
        else if &self.current_tool == "p"{
            self.rectangle(main_window, &gpos, &self.mstart_gpos)
        }
        else if &self.current_tool == "e"{
            self.ellipse(main_window, &gpos, &self.mstart_gpos);
        }
        else if &self.current_tool == "w"{
            self.filled_ellipse(main_window, &gpos, &self.mstart_gpos);
        }

        let mut render_change = false;
        if click_down{
            render_change = true;
        }
        if self.prev_gpos != gpos{
            render_change = true;
        }
        self.prev_gpos = gpos;
        return render_change;
    }


    pub fn line(&self, main_window: &mut MainWindow<'_>,
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
                main_window.add_to_preview_buffer(begin_row, begin_col, self.current_key);    
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
    
    pub fn rectangle(&self, main_window: &mut MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]){
    
        main_window.preview_buffer.clear();
        //4 lines
        self.line(main_window,
                  &[start_mouse_pos[0], start_mouse_pos[1]], //(s,s) to (c,s)
                  &[current_mouse_pos[0], start_mouse_pos[1]], //top left to bottom left
                  false);
        self.line(main_window,
                  &[start_mouse_pos[0], start_mouse_pos[1]], //(s,s) to (s,c)
                  &[start_mouse_pos[0], current_mouse_pos[1]], //top left to top right
                  false);
        self.line(main_window,
                  &[start_mouse_pos[0], current_mouse_pos[1]], //(s,c) to (c,c)
                  &[current_mouse_pos[0], current_mouse_pos[1]], //top right to bottom right
                  false);
        self.line(main_window,
                  &[current_mouse_pos[0], start_mouse_pos[1]], //(c,s) to (c,c)
                  &[current_mouse_pos[0], current_mouse_pos[1]], //bottom left to bottom right
                  false);
    }
    
    pub fn filled_rectangle(&self, main_window: &mut MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {
    
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
            self.line(main_window,
            &[row_num, begin_col], //further iterates those lines horizontally (left to right or right to left)
            &[row_num, fin_col],
            false);
        }
    }
    
    pub fn circle(&self, main_window: &mut MainWindow<'_>,
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
            main_window.add_to_preview_buffer(begin_row + row_num, begin_col + col_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row + col_num, begin_col + row_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row - col_num, begin_col + row_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row - row_num, begin_col + col_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row - row_num, begin_col - col_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row - col_num, begin_col - row_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row + col_num, begin_col - row_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row + row_num, begin_col - col_num, self.current_key);
    
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
    
    pub fn filled_circle(&self, main_window: &mut MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) { //basically, just fill in line tools, trig if necessary
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
    
            self.line(main_window,
            &[(begin_row + row_num), (begin_col + col_num)], 
            &[(begin_row - row_num), (begin_col + col_num)],
            false);
    
            self.line(main_window,
            &[(begin_row + col_num), (begin_col + row_num)], 
            &[(begin_row - col_num), (begin_col + row_num)],
            false);
    
            self.line(main_window,
            &[(begin_row + row_num), (begin_col - col_num)], 
            &[(begin_row - row_num), (begin_col - col_num)],
            false);
    
            self.line(main_window,
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
    
    fn draw_ellipse<F>(&self, main_window: &mut MainWindow<'_>, mut render_func: F, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) //necessary for ellipse and filled_ellipse
        //same credits (docs.rs) but draw_ellipse ofc
        //this func is the meat for drawing the ellipse, ellipse and filled_ellipse tool just call specialized versions of this
        where
        F: FnMut(&mut MainWindow<'_>, i32, i32, i32, i32), {
    
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
                p_row += 2.0 * col_diff_squared;
                p += row_diff_squared - p_col + p_row;
            }
            render_func(main_window, begin_row, begin_col, row_num, col_num);
        }
    }
    
    pub fn ellipse(&self, main_window: &mut MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {
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
            self.circle(main_window,
            &[begin_row, begin_col],
            &[fin_row, fin_col]);
            return;
        }
        else if col_dif == 0 { //Straight line is faster
        self.line(main_window,
        &[begin_row + (-2 * (begin_row - fin_row)), begin_col],
        &[begin_row, begin_col],
        true);
        return; //only do straight line, no ellipse at all
        }
        else if row_dif == 0 { 
        self.line(main_window,
        &[fin_row, begin_col + (-2 * (begin_col - fin_col))],
        &[begin_row, begin_col],
        true); 
        return;  
        }
        //passed to draw_ellipse
        let draw_quad_pixels = |main_window: &mut MainWindow<'_>, begin_row: i32, begin_col: i32, row_num: i32, col_num: i32| {
            //mentioned in previous credits's source, but I figured I'd be specific https://web.archive.org/web/20160128020853/http://tutsheap.com/c/mid-point-ellipse-drawing-algorithm/
            //draw_quad_pixels in doc.rs
            main_window.add_to_preview_buffer(begin_row + row_num, begin_col + col_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row - row_num, begin_col + col_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row + row_num, begin_col - col_num, self.current_key);
            main_window.add_to_preview_buffer(begin_row - row_num, begin_col - col_num, self.current_key);
        };
    
        self.draw_ellipse(main_window,
        draw_quad_pixels,
        &[begin_row, begin_col],
        &[fin_row, fin_col]);
    }
    
    pub fn filled_ellipse(&self, main_window: &mut MainWindow<'_>, current_mouse_pos: &[i32; 2], start_mouse_pos: &[i32; 2]) {
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
            self.filled_circle(main_window,
            &[begin_row, begin_col],
            &[fin_row, fin_col]);
            return;
        }
        
    
        else if col_dif == 0 { //Straight line is faster
        self.line(main_window,
            &[begin_row + (-2 * (begin_row - fin_row)), begin_col],
            &[begin_row, begin_col],
            true);
            return; //only do straight line, no ellipse at all
        }
            else if row_dif == 0 { 
            self.line(main_window,
            &[fin_row, begin_col + (-2 * (begin_col - fin_col))],
            &[begin_row, begin_col],
            true); 
            return;  
        }
        //will be passed to draw_ellipse to draw line pair when drawing
        let draw_line_pairs = |main_window: &mut MainWindow<'_>, begin_row: i32, begin_col: i32, row_num: i32, col_num: i32| {
            self.line(
                main_window,
                &[(begin_row - row_num), (begin_col + col_num)],
                &[(begin_row + row_num), (begin_col + col_num)],
                false );
            self.line(
                main_window,
                &[(begin_row - row_num), (begin_col - col_num)],
                &[(begin_row + row_num) , (begin_col - col_num)],
                false );
        };
    
        self.draw_ellipse(main_window,
            draw_line_pairs,
            &[begin_row, begin_col],
            &[fin_row, fin_col]);
    }
    
    pub fn text(&mut self, main_window: &mut MainWindow<'_>, input: &String, action: &str){
        if action == "escape"{
            self.current_tool = String::from("f");
        }
        else if action == "backspace"{
            let mut end_offset = 0;
            if self.prev_gpos[1] == (main_window.num_of_cols as i32) - 1 && //updates our position
            main_window.window_array[self.prev_gpos[0] as usize][self.prev_gpos[1] as usize] != ' '{ //moves to that new position
                end_offset = 1;
            }
            main_window.window_array[self.prev_gpos[0] as usize][(std::cmp::max(self.prev_gpos[1]-1+end_offset, 0)) as usize] = ' ';
            self.prev_gpos = [self.prev_gpos[0], std::cmp::max(self.prev_gpos[1] - 1 + end_offset, 0)];
            
        }
        else if action == "up"{
            self.prev_gpos = [std::cmp::max(self.prev_gpos[0]-1, 0), self.prev_gpos[1]];
        }
        else if action == "down"{
            self.prev_gpos = [std::cmp::min(self.prev_gpos[0]+1, (main_window.num_of_rows as i32)-1), self.prev_gpos[1]];
        }
        else if action == "left"{
            self.prev_gpos = [self.prev_gpos[0], std::cmp::max(self.prev_gpos[1]-1, 0)];
        }
        else if action == "right"{
            self.prev_gpos = [self.prev_gpos[0], std::cmp::min(self.prev_gpos[1]+1, (main_window.num_of_cols as i32)-1)];
        }
        else{
            let text_vec: Vec<char> = input.chars().collect();
            if self.prev_gpos[1] >= (main_window.num_of_cols as i32) {
                main_window.window_array[self.prev_gpos[0] as usize][(main_window.num_of_cols - 1) as usize] = text_vec[0];
            }
            else {
                main_window.window_array[self.prev_gpos[0] as usize][self.prev_gpos[1] as usize] = text_vec[0];
            }
            self.prev_gpos = [self.prev_gpos[0], std::cmp::min(self.prev_gpos[1]+1, (main_window.num_of_cols as i32)-1)];
        }
    }
    
    pub fn free(&self, main_window: &mut MainWindow<'_>, current_mouse_pos: &[i32; 2], prev_gpos: &[i32; 2]){
        if current_mouse_pos != prev_gpos{
            main_window.add_to_preview_buffer(current_mouse_pos[0], current_mouse_pos[1], self.current_key);
        }
    }
}
