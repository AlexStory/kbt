use std::{env, fs::OpenOptions};

use crate::kanban;

pub(crate) fn get_path() -> String {
    let path = env::current_exe().unwrap();
    let path = path.parent().unwrap();
    path.join("board.json").to_str().unwrap().to_string()
}

pub(crate) fn read_board() -> Result<kanban::Board, Box<dyn::std::error::Error>> {
    let path = get_path();
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

   match serde_json::from_reader(f) {
       Ok(board) => Ok(board),
       Err(e) if e.is_eof() => Ok(kanban::Board::new()),
       Err(e) => panic!("{e}"),
   }
}

pub fn write_board(board: &kanban::Board) -> Result<(), std::io::Error> {
    let path = get_path();
    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    serde_json::to_writer_pretty(f, &board)?;
    Ok(())
}