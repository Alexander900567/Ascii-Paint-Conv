
use std::collections::VecDeque;

pub struct UndoRedo{
        
    pub undo_buffer: VecDeque<Vec<(i32, i32, char)>>,
    pub redo_buffer: VecDeque<Vec<(i32, i32, char)>>,
    pub max_undo_history: usize,
}

impl UndoRedo{
    pub fn new() -> UndoRedo{
        UndoRedo{
            undo_buffer: VecDeque::new(),
            redo_buffer: VecDeque::new(),
            max_undo_history: 50,
        }
    }

    pub fn add_to_undo(&mut self, preview_buffer: &Vec<(i32, i32, char)>, window_array: &Vec<Vec<char>>){
        let mut change = Vec::new();
        for grid in preview_buffer{
            change.push((grid.0, grid.1, window_array[grid.0 as usize][grid.1 as usize]));
        }

        self.undo_buffer.push_front(change);

        if self.undo_buffer.len() > self.max_undo_history{
            let _ = self.undo_buffer.pop_back();
        }
    }

    pub fn perform_undo(&mut self, window_array: &mut Vec<Vec<char>>){
        let undo = self.undo_buffer.pop_front().unwrap_or(Vec::new());
        if undo.len() < 1{return;} //eject if there's nothing to undo
        let mut redo = Vec::new();
        
        //has to be two loops or else repeat preview buffer entries wreck stuff
        for grid in &undo{
            redo.push((grid.0, grid.1, window_array[grid.0 as usize][grid.1 as usize]));
        }
        for grid in &undo{
            window_array[grid.0 as usize][grid.1 as usize] = grid.2;
        }
        self.redo_buffer.push_front(redo); 
    }

    pub fn perform_redo(&mut self, window_array: &mut Vec<Vec<char>>){
        let redo = self.redo_buffer.pop_front().unwrap_or(Vec::new());
        if redo.len() < 1{return;} //eject if there's nothing to redo
        let mut undo = Vec::new();
        
        //has to be two loops or else repeat preview buffer entries wreck stuff
        for grid in &redo{
            undo.push((grid.0, grid.1, window_array[grid.0 as usize][grid.1 as usize]));
        }
        for grid in &redo{
            window_array[grid.0 as usize][grid.1 as usize] = grid.2;
        }
        self.undo_buffer.push_front(undo);    
    }
}
