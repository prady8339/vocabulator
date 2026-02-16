use crate::db::models::Word;

pub struct Session {
    pub words: Vec<Word>,
    pub index: usize,

    // UI state
    pub show_definition: bool,
    pub graded: Option<bool>,
    pub input_buffer: String,
    pub insert_mode: bool,
}

impl Session {
    pub fn current(&self) -> &Word {
        &self.words[self.index]
    }

    pub fn current_mut(&mut self) -> &mut Word {
        &mut self.words[self.index]
    }

    pub fn reset_ui_state(&mut self) {
        self.show_definition = false;
        self.graded = None;
        self.input_buffer.clear();
        self.insert_mode = false;
    }
}
