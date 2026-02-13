use crate::db::models::Word;

#[derive(Debug, Clone)]
pub struct Session {
    pub words: Vec<Word>,
    pub index: usize,
}
