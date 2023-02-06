#![warn(clippy::all, clippy::pedantic, clippy::perf)]
use clap::{Parser, Subcommand};

mod commands;
mod kanban;
mod persist;

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(arg_required_else_help = true)]
enum Commands {
    /// Manage lanes
    Lanes {
        #[clap(subcommand)]
        command: Option<LaneCommands>,
    },
}

#[derive(Subcommand)]
enum LaneCommands {
    /// List all lanes
    List,
    /// View a lane by name
    View {
        /// The name of the lane to view
        name: String,
    },
    /// Create a new lane
    Create {
        /// The name of the lane to create
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    let board = match persist::read_board() {
        Ok(b) => b,
        Err(e) => panic!("{e}"),
    };

    if let Some(Commands::Lanes { command }) = args.command {
        handle_lanes(command, board);
    }
}

fn handle_lanes(command: Option<LaneCommands>, board: kanban::Board) {
    match command {
        Some(LaneCommands::List) => commands::lane::handle_list(board.lanes),
        Some(LaneCommands::View { name }) => {
            commands::lane::handle_view(board.lanes.into_iter().find(|lane| lane.name == name))
        }
        Some(LaneCommands::Create { name }) => commands::lane::handle_create(board, &name),
        None => {}
    }
}
