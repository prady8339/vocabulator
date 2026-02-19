use super::models::Word;
use crate::ui::app::Screen;
use anyhow::Result;
use rusqlite::{Connection, params};

fn screen_to_int(screen: Screen) -> i32 {
    match screen {
        Screen::Practice => 0,
        Screen::Test => 1,
        _ => 0,
    }
}

fn int_to_screen(v: i32) -> Screen {
    match v {
        1 => Screen::Test,
        _ => Screen::Practice,
    }
}

pub fn fetch_progress(conn: &Connection) -> Result<(Screen, i32, usize)> {
    let mode: i32 = conn
        .query_row(
            "SELECT value FROM app_state WHERE key=?1",
            params!["mode"],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let group_id: i32 = conn
        .query_row(
            "SELECT value FROM app_state WHERE key=?1",
            params!["group_id"],
            |row| row.get(0),
        )
        .unwrap_or(1);

    let index: i32 = conn
        .query_row(
            "SELECT value FROM app_state WHERE key=?1",
            params!["index"],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok((int_to_screen(mode), group_id, index as usize))
}
