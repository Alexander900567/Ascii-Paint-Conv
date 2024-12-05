use crate::main_window::MainWindow;
use crate::gui::Gui;
use std::cmp::min;
use std::cmp::max;

pub struct RectangleSelector{
    pub active: bool,
    pub start_gpos: (i32, i32),
    pub top_left: (i32, i32),
    pub bot_right: (i32, i32),
    pub size: (i32, i32),
    original_buffer: Vec<(i32, i32, char)>
}



impl RectangleSelector{
    pub fn new() -> RectangleSelector{
        RectangleSelector{
            active: false,
            start_gpos: (-1, -1),
            top_left: (-1, -1),
            bot_right: (-1, -1),
            size: (-1, -1),
            original_buffer: Vec::new(),
        }
    }
    
    pub fn on_mouse_down(&mut self, gpos: &[i32; 2]){
        if !self.active{
            self.start_gpos = (gpos[0], gpos[1]);
            self.change_corners(&(gpos[0], gpos[1]));
        }
        else{
            self.start_gpos = (gpos[0], gpos[1]);
        }
    }


    pub fn on_mouse_move(&mut self, main_window: &mut MainWindow<'_>, gpos: &[i32; 2]){
        if !self.active{
            self.change_corners(&(gpos[0], gpos[1])); 
        }
        else if gpos != &[self.start_gpos.0, self.start_gpos.1]{
            let mut row_delta: i32 = gpos[0] - self.start_gpos.0;
            let mut col_delta: i32 = gpos[1] - self.start_gpos.1;

            let mut bound_deltas = |corner: (i32, i32)| {
                if row_delta + corner.0 < 0 {row_delta = corner.0 * -1;}
                else if row_delta + corner.0 >= main_window.num_of_rows as i32{
                    row_delta = main_window.num_of_rows as i32 - 1 - corner.0;
                }
                if col_delta + corner.1 < 0 {col_delta = corner.1 * -1;}
                else if col_delta + corner.1 >= main_window.num_of_cols as i32{
                    col_delta = main_window.num_of_cols as i32 - 1 - corner.1;
                }
                
            };

            bound_deltas(self.top_left);
            bound_deltas(self.bot_right);
            
            self.top_left = (self.top_left.0 + row_delta, self.top_left.1 + col_delta);
            self.bot_right = (self.bot_right.0 + row_delta, self.bot_right.1 + col_delta);
            self.start_gpos = (gpos[0], gpos[1]);
            if row_delta != 0 || col_delta != 0{
                for item in main_window.preview_buffer.iter_mut(){
                    item.0 = item.0 + row_delta;
                    item.1 = item.1 + col_delta;
                }
            }
        }
    }

    pub fn on_mouse_up(&mut self, main_window: &mut MainWindow<'_>, gui_bar: &mut Gui){
        if !self.active{
            self.active = true;
            for row in self.top_left.0..=self.bot_right.0{
                for col in self.top_left.1..=self.bot_right.1{
                    main_window.add_to_preview_buffer(row, col, main_window.window_array[row as usize][col as usize]);
                    self.original_buffer.push((row, col, main_window.window_array[row as usize][col as usize]));
                    main_window.window_array[row as usize][col as usize] = ' ';
                }
            }
            gui_bar.hide_button(7);
            gui_bar.hide_button(8);
        }
    }

    pub fn reset_box(&mut self, main_window: &mut MainWindow<'_>, gui_bar: &mut Gui){
        self.active = false;
        self.top_left = (-1, -1);
        self.bot_right = (-1, -1);
        self.size = (-1, -1);
        self.start_gpos = (-1, -1);

        let mut change: Vec<(i32, i32, char)> = Vec::new();
        for grid in &main_window.preview_buffer{
            change.push((grid.0, grid.1, main_window.window_array[grid.0 as usize][grid.1 as usize]));
        }
        for grid in &self.original_buffer{
            change.push((grid.0, grid.1, grid.2));
        }
        main_window.undo_redo.undo_buffer.push_front(change);

        main_window.write_buffer(false); 

        gui_bar.show_button(7);
        gui_bar.show_button(8);
    }

    fn change_corners(&mut self, new_gpos: &(i32, i32)){
        self.top_left = (min(self.start_gpos.0, new_gpos.0), min(self.start_gpos.1, new_gpos.1));
        self.bot_right = (max(self.start_gpos.0, new_gpos.0), max(self.start_gpos.1, new_gpos.1));
        self.size = (self.bot_right.0 - self.top_left.0 + 1, self.bot_right.1 - self.top_left.1 + 1);
    }






}
