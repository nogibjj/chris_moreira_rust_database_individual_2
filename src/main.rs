use chris_moreira_rust_database_individual_2::{
    create_table, extract, load_data_from_csv, query_delete, query_read, query_update,
};
use clap::{Parser, ValueEnum};
use rusqlite::Connection;
use std::time::Instant;

/// Enum to define possible CLI actions
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Action {
    Extract,
    Create,
    Load,
    QueryRead,
    QueryUpdate,
    QueryDelete,
}

/// CLI arguments structure
#[derive(Parser)]
#[command(name = "Rust SQLite CLI")]
#[command(
    about = "A CLI tool for SQLite operations with Rust",
    long_about = None
)]
struct Cli {
    #[arg(value_enum)]
    action: Action,
}

fn main_results(conn: &Connection, action: Action) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        Action::Extract => {
            println!("Extracting data...");
            let url = concat!(
                "https://raw.githubusercontent.com/nogibjj/",
                "chris_moreira_week5_python_sql_db_project/",
                "main/data/Spotify_Most_Streamed_Songs.csv"
            );
            let result = extract(url, "../data/Spotify_Most_Streamed_Songs.csv");
            println!("Extract Result: {:?}", result);
        }
        Action::Create => {
            println!("Creating table...");
            let start_time = Instant::now();
            let result = create_table(conn);
            let duration = start_time.elapsed();
            println!(
                "Create Table Result: {:?}, Duration: {:?}",
                result, duration
            );
        }
        Action::Load => {
            println!("Loading data...");
            let result = load_data_from_csv(conn, "../data/Spotify_Most_Streamed_Songs.csv");
            println!("Load Result: {:?}", result);
        }
        Action::QueryRead => {
            println!("Reading data...");
            let result = query_read(conn);
            println!("Read Result: {:?}", result);
        }
        Action::QueryUpdate => {
            println!("Updating data...");
            let result = query_update(conn);
            println!("Update Result: {:?}", result);
        }
        Action::QueryDelete => {
            println!("Deleting data...");
            let result = query_delete(conn);
            println!("Delete Result: {:?}", result);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let conn = Connection::open("SpotifyDB.db")?;
    main_results(&conn, cli.action)?;
    Ok(())
}
