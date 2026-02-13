use super::session::Session;

pub fn start_session() -> Session {
    Session {
        words: vec![],
        index: 0,
    }
}
