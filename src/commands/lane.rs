use crate::{
    kanban::{Board, Lane},
    persist,
};

pub fn handle_list(lanes: Vec<Lane>) {
    if lanes.is_empty() {
        println!("No lanes found");
    } else {
        println!("Lanes:");
        for lane in lanes {
            println!(" - {}", lane.name);
        }
    }
}

pub fn handle_view(lane: Option<Lane>) {
    match lane {
        Some(lane) => {
            println!("Lane: {}", lane.name);
            for card in lane.cards {
                println!("\t{}", card.title);
            }
        }
        None => println!("No lane found"),
    }
}

pub fn handle_create(mut board: Board, name: &str) {
    let lane = Lane::new(name);
    board.add_lane(lane);
    match persist::write_board(&board) {
        Ok(_) => println!("Created lane: {name}"),
        Err(e) => println!("Failed to create lane: {e}"),
    }
}
