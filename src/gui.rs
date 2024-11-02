use crate::main_window;

pub struct Gui{
    pub gui_grid: Vec<Vec<i32>>,
    pub buttons: std::collections::HashMap<i32, Button>,
    pub toggle_groups: std::collections::HashMap<i32, i32>,
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
        /*
        making a button
        (button_id, Button::new(&mut start_grid, &mut start_groups,
                                button_id, top_left, bottom_right,
                                button_label, is_pressed, toggle_group, visible)

        button_id: i32 = number that represents the button, do not use -1 or the same value for more than one button
        top_left: (i32, i32) = the grid position of the top left of the button
        bottom_right: (i32, i32) = the grid position of the bottom right of the button
        button_label: &str = the text to be rendered on the button
        is_pressed: i32 = does the button start toggled on(1) or off(0), -1 makes the button a oneshot (no toggle state)
        toggle_group: i32 = only one button from a toggle group can be on at once, -1 makes the button independent of all others 
        visible: bool = does the button start visible on the bar
        */
        let mut start_buttons = std::collections::HashMap::new();
        let mut start_groups = std::collections::HashMap::new();
        start_buttons.insert(0, Button::new(&mut start_grid, &mut start_groups,
                                            0, (1, 1), (3, 2),
                                            "free", 1, 0, true));
        
        start_buttons.insert(1, Button::new(&mut start_grid, &mut start_groups,
                                            1, (1, 12), (3, 13),
                                            "fill", 0, -1, false));

        start_buttons.insert(2, Button::new(&mut start_grid, &mut start_groups,
                                            2, (5, 1), (7, 2),
                                            "line", 0, 0, true));

        start_buttons.insert(3, Button::new(&mut start_grid, &mut start_groups,
                                            3, (5, 4), (7, 5), 
                                            "rectangle", 0, 0, true));

        start_buttons.insert(4, Button::new(&mut start_grid, &mut start_groups,
                                            4, (1, 4), (3, 5),
                                            "circle", 0, 0, true));

        start_buttons.insert(5, Button::new(&mut start_grid, &mut start_groups, 
                                            5, (1, 7), (3, 8),
                                            "text", 0, 0, true));

        start_buttons.insert(6, Button::new(&mut start_grid, &mut start_groups, 
                                            6, (5, 7), (7, 8),
                                            "picture", 0, 0, true));

        start_buttons.insert(7, Button::new(&mut start_grid, &mut start_groups, 
                                            7, (1, 17), (3, 17),
                                            "<-", -1, 0, true));

        start_buttons.insert(8, Button::new(&mut start_grid, &mut start_groups, 
                                            8, (1, 19), (3, 19),
                                            "->", -1, 0, true));

        start_buttons.insert(9, Button::new(&mut start_grid, &mut start_groups, 
                                            9, (5, 19), (7, 19),
                                            "co", -1, 0, true));

        start_buttons.insert(10, Button::new(&mut start_grid, &mut start_groups, 
                                            10, (1, 12), (3, 12),
                                            "1", 0, 1, false));

        start_buttons.insert(11, Button::new(&mut start_grid, &mut start_groups, 
                                            11, (1, 13), (3, 13),
                                            "2", 0, 1, false));

        start_buttons.insert(12, Button::new(&mut start_grid, &mut start_groups, 
                                            12, (1, 14), (3, 14),
                                            "3", 0, 1, false));

        start_buttons.insert(13, Button::new(&mut start_grid, &mut start_groups, 
                                            13, (1, 15), (3, 15),
                                            "4", 1, 1, false));

        start_buttons.insert(14, Button::new(&mut start_grid, &mut start_groups, 
                                            14, (5, 12), (7, 13),
                                            "edge", 0, -1, false));
        println!("{:?}", start_groups);
        Gui{
            gui_grid: start_grid,
            buttons: start_buttons,
            toggle_groups: start_groups,
            num_rows: num_rows,
            num_cols: num_cols,
            row_size: (gui_height as i32 / num_rows) as f32,
            col_size: (window_width as i32 / num_cols) as f32,

        }
    }

    pub fn handle_gui_click(&mut self, x: i32, y: i32, main_window: &mut main_window::MainWindow<'_>, current_tool: &mut String, tool_modifier: &mut Vec<String>){
        let grid_pos = self.get_gui_grid_pos(x, y);
        
        let clicked_id = self.gui_grid[grid_pos.0 as usize][grid_pos.1 as usize];
        if clicked_id == -1{
            return;
        }
        let clicked_is_pressed = self.buttons.get(&clicked_id).unwrap().is_pressed;
        let toggle_group = self.buttons.get(&clicked_id).unwrap().toggle_group;

        let mut unclick_id = 0;
        if clicked_is_pressed != -1 {
            unclick_id = self.handle_unclick(toggle_group, clicked_id, tool_modifier);
        }

        if clicked_id == 0{
            self.click_free(current_tool);
        }
        else if clicked_id == 2{
            self.click_line(current_tool);
        }
        else if clicked_id == 3{
            self.click_rect(current_tool);
        }
        else if clicked_id == 4{
            self.click_circle(current_tool);
        }
        else if clicked_id == 5{
            self.click_text(current_tool);
        }
        else if clicked_id == 6{
            self.click_picture(current_tool);
        }
        else if clicked_id == 7{
            self.click_undo(main_window);
        }
        else if clicked_id == 8{
            self.click_redo(main_window);
        }
        else if clicked_id == 9{
            self.click_copy_to_clipboard(main_window);
        }
        else if Vec::from([10, 11, 12, 13]).contains(&clicked_id){
            self.click_ascii_pallete(tool_modifier, clicked_id);
        }

        if toggle_group == -1 && clicked_is_pressed == 0{
            if clicked_id == 1{
                self.click_fill(tool_modifier);
            }
            if clicked_id == 14{
                self.click_edge(tool_modifier);
            }
        }

        if clicked_is_pressed != -1 && toggle_group != -1{
            self.buttons.get_mut(&unclick_id).unwrap().is_pressed = 0;
            self.buttons.get_mut(&clicked_id).unwrap().is_pressed = 1;
            self.toggle_groups.insert(toggle_group, clicked_id);
        }
        else if toggle_group == -1 && clicked_is_pressed == 0{
            self.buttons.get_mut(&clicked_id).unwrap().is_pressed = 1;
        }
        else if toggle_group == -1 && clicked_is_pressed == 1{
            self.buttons.get_mut(&clicked_id).unwrap().is_pressed = 0;
        }
    }

    pub fn handle_unclick(&mut self, toggle_group: i32, clicked_id: i32, tool_modifier: &mut Vec<String>) -> i32{
        let unclick_id: i32; 
        let pressed_status: i32;
        if toggle_group == -1{
            unclick_id = clicked_id;
            pressed_status = self.buttons.get(&clicked_id).unwrap().is_pressed;
        }
        else{
            unclick_id = *self.toggle_groups.get(&toggle_group).unwrap();
            pressed_status = -1;
        }
        
        if unclick_id == 3{
            self.unclick_rect(); 
        }
        else if unclick_id == 4{
            self.unclick_circle();
        }
        else if unclick_id == 6{
            self.unclick_picture();
        }

        if toggle_group == -1 && pressed_status == 1{
            if unclick_id == 1{
                self.unclick_fill(tool_modifier);
            }
            if unclick_id == 14{
                self.unclick_edge(tool_modifier);
            }
        }

        return unclick_id;
    }

    pub fn get_gui_grid_pos(&self, x: i32, y: i32) -> (i32, i32){
        let mut row_pos = (y as f32 / self.row_size) as i32;
        let mut col_pos = (x as f32 / self.col_size) as i32;

        if row_pos >= self.num_rows {row_pos = self.num_rows - 1;}
        else if row_pos < 0 {row_pos = 0;}
        if col_pos >= self.num_cols {col_pos = self.num_cols - 1;}
        else if col_pos < 0 {col_pos = 0;}

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
    
    fn click_free(&self, current_tool: &mut String){
        *current_tool = String::from('f');
    }

    fn click_line(&self, current_tool: &mut String){
        *current_tool = String::from('l');
    }

    fn click_rect(&mut self, current_tool: &mut String){
        self.show_button(1);
        *current_tool = String::from('r');
    }
    fn unclick_rect(&mut self){
        self.hide_button(1);
    }

    fn click_circle(&mut self, current_tool: &mut String){
        self.show_button(1);
        *current_tool = String::from('o');
    }
    fn unclick_circle(&mut self){
        self.hide_button(1);
    }

    fn click_fill(&self, tool_modifier: &mut Vec<String>){
        tool_modifier[2] = String::from("a");
    }
    fn unclick_fill(&self, tool_modifier: &mut Vec<String>){
        tool_modifier[2] = String::from(" ");
    }

    fn click_ascii_pallete(&self, tool_modifier: &mut Vec<String>, clicked_id: i32){
        if clicked_id == 10{
            tool_modifier[0] = String::from("1");
        }
        else if clicked_id == 11{
            tool_modifier[0] = String::from("2");
        }
        else if clicked_id == 12{
            tool_modifier[0] = String::from("3");
        }
        else if clicked_id == 13{
            tool_modifier[0] = String::from("4");
        }
    }

    fn click_edge(&self, tool_modifier: &mut Vec<String>){
        tool_modifier[1] = String::from("l");
    }
    fn unclick_edge(&self, tool_modifier: &mut Vec<String>){
        tool_modifier[1] = String::from(" ");
    }

    fn click_text(&self, current_tool: &mut String){
        *current_tool = String::from('t');
    }

    fn click_picture(&mut self, current_tool: &mut String){
        self.show_button(10);
        self.show_button(11);
        self.show_button(12);
        self.show_button(13);
        self.show_button(14);
        *current_tool = String::from('p');
    }
    fn unclick_picture(&mut self){
        self.hide_button(10);
        self.hide_button(11);
        self.hide_button(12);
        self.hide_button(13);
        self.hide_button(14);
    }

    fn click_undo(&mut self, main_window: &mut main_window::MainWindow<'_>){
        main_window.undo_redo.perform_undo(&mut main_window.window_array);
    }

    fn click_redo(&mut self, main_window: &mut main_window::MainWindow<'_>){
        main_window.undo_redo.perform_redo(&mut main_window.window_array);
    }

    fn click_copy_to_clipboard(&mut self, main_window: &mut main_window::MainWindow<'_>){
        main_window.copy_to_clipboard();
    }
}

pub struct Button{
    pub button_id: i32,
    pub top_left: (i32, i32),
    pub bottom_right: (i32, i32),
    pub grid_pos: Vec<(i32, i32)>,
    pub button_label: String,
    pub is_pressed: i32, //0 unpressed, 1 pressed, -1 one-shot button
    pub toggle_group: i32, //-1 toggle
    pub visible: bool,
}

impl Button{
    pub fn new(
        gui_grid: &mut Vec<Vec<i32>>,
        toggle_groups: &mut std::collections::HashMap<i32, i32>,
        button_id: i32,
        top_left: (i32, i32),
        bottom_right: (i32, i32),
        button_label: &str,
        is_pressed: i32, //0 unpressed, 1 pressed, -1 one-shot button
        toggle_group: i32,
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

        if is_pressed == 1 && toggle_group != -1{
            toggle_groups.insert(toggle_group, button_id);
        }
        else if toggle_group > -1 && is_pressed != -1{
            if !toggle_groups.contains_key(&toggle_group){
                toggle_groups.insert(toggle_group, -1);
            }
        }

        Button{
            button_id: button_id,
            top_left: top_left,
            bottom_right: bottom_right,
            grid_pos: start_pos,
            button_label: String::from(button_label),
            is_pressed: is_pressed,
            toggle_group: toggle_group,
            visible: visible,
        }
    }
}
 

