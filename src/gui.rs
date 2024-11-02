pub struct Gui{
    pub gui_grid: Vec<Vec<i32>>,
    pub buttons: std::collections::HashMap<i32, Button>,
    pub num_rows: i32,
    pub num_cols: i32,
    pub row_size: f32,
    pub col_size: f32,
}

impl Gui{
    pub fn new(
        gui_height: u32,
        window_width: u32,
        num_rows: i32,
        num_cols: i32
        ) -> Gui{

        let mut start_grid = Vec::new();
        for _ in 0..num_rows{
            let mut temp = Vec::new();
            for _ in 0..num_cols{
                temp.push(-1);
            }
            start_grid.push(temp);
        }

        let mut start_buttons = std::collections::HashMap::new();
        start_buttons.insert(0, Button::new(&mut start_grid, 0,
                                        (1, 1), (3, 2),
                                        "line", 0, true));
        
        start_buttons.insert(1, Button::new(&mut start_grid, 1,
                                        (1, 4), (3, 5),
                                        "test", -1, false));

        start_buttons.insert(2, Button::new(&mut start_grid, 2,
                                        (1, 8), (3, 10),
                                        "line", 1, true));

        Gui{
            gui_grid: start_grid,
            buttons: start_buttons,
            num_rows: num_rows,
            num_cols: num_cols,
            row_size: (gui_height as i32 / num_rows) as f32,
            col_size: (window_width as i32 / num_cols) as f32,

        }
    }

    pub fn handle_gui_click(&mut self, x: i32, y: i32){
        let grid_pos = self.get_gui_grid_pos(x, y);
        
        let clicked_id = self.gui_grid[grid_pos.0 as usize][grid_pos.1 as usize];
        if clicked_id == -1{
            return;
        }
        let clicked_is_pressed = self.buttons.get(&clicked_id).unwrap().is_pressed;
     
        let mut unclick_id = 0;
        if clicked_is_pressed != -1{
            unclick_id = self.handle_unclick();
        }

        if clicked_id == 0{
            self.click_line();
        }

        if clicked_is_pressed != -1{
            self.buttons.get_mut(&unclick_id).unwrap().is_pressed = 0;
            self.buttons.get_mut(&clicked_id).unwrap().is_pressed = 1;
        }
    }

    pub fn handle_unclick(&mut self) -> i32{
        let mut unclick_id = 0;
        for button in self.buttons.values(){
            if button.is_pressed == 1{
                unclick_id = button.button_id;
            }
        }
        
        if unclick_id == 0{
            self.unclick_line(); 
        }
            

        return unclick_id;
    }

    pub fn get_gui_grid_pos(&self, x: i32, y: i32) -> (i32, i32){
        let row_pos = (y as f32 / self.row_size) as i32;
        let col_pos = (x as f32 / self.col_size) as i32;
        return (row_pos, col_pos);
    }

    pub fn hide_button(&mut self, button_id: i32){
        self.buttons.get_mut(&button_id).unwrap().visible = false;
        let grid_pos = &self.buttons.get(&button_id).unwrap().grid_pos;

        for pos in grid_pos{
            self.gui_grid[pos.0 as usize][pos.1 as usize] = -1;
        }
    }

    pub fn show_button(&mut self, button_id: i32){
        self.buttons.get_mut(&button_id).unwrap().visible = true;
        let grid_pos = &self.buttons.get(&button_id).unwrap().grid_pos;

        for pos in grid_pos{
            self.gui_grid[pos.0 as usize][pos.1 as usize] = button_id;
        }
    }

    //------------------button functions
    
    fn click_line(&mut self){
        self.show_button(1);
    }

    fn unclick_line(&mut self){
        self.hide_button(1);
    }


}

/*

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

*/

pub struct Button{
    pub button_id: i32,
    pub top_left: (i32, i32),
    pub bottom_right: (i32, i32),
    pub grid_pos: Vec<(i32, i32)>,
    pub button_label: String,
    pub is_pressed: i32, //0 unpressed, 1 pressed, -1 one-shot button
    pub visible: bool,
}

impl Button{
    pub fn new(
        gui_grid: &mut Vec<Vec<i32>>,
        button_id: i32,
        top_left: (i32, i32),
        bottom_right: (i32, i32),
        button_label: &str,
        is_pressed: i32, //0 unpressed, 1 pressed, -1 one-shot button
        visible: bool,
        ) -> Button{

        let mut start_pos = Vec::new();
        for row in top_left.0..=bottom_right.0{
            for col in top_left.1..=bottom_right.1{
                start_pos.push((row, col));
                if visible{
                    gui_grid[row as usize][col as usize] = button_id; 
                }
            }
        }

        Button{
            button_id: button_id,
            top_left: top_left,
            bottom_right: bottom_right,
            grid_pos: start_pos,
            button_label: String::from(button_label),
            is_pressed: is_pressed,
            visible: visible,
        }
    }
}
 

