use crate::{
    kanban::{Board, Card},
    persist,
};

pub fn handle_create(mut board: Board, title: &str, lane: &str) {
    let card = Card::new(title);

    if let Some(l) = board.lanes.iter_mut().find(|l| l.name == *lane) {
        l.add(card);
    } else {
        println!("Lane does not exist");
        return;
    }

    match persist::write_board(&board) {
        Ok(_) => println!("Added {title} to {lane}"),
        Err(_) => println!("Failed to save card"),
    };
}
