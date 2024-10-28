use chris_moreira_rust_database_individual_2::{
    extract, create_table, load_data_from_csv, query_create,
    query_read,
};
use rusqlite::Connection;

#[test]
fn test_extract() {
    let url = "https://raw.githubusercontent.com/nogibjj/\
        chris_moreira_week5_python_sql_db_project/main/\
        data/Spotify_Most_Streamed_Songs.csv";
    let file_path = "../data/Spotify_Most_Streamed_Songs.csv";
    
    let result = extract(url, file_path);
    assert!(result.is_ok(), "Extract function failed");

    assert!(std::fs::metadata(file_path).is_ok(),
        "File not found after extraction");
}

#[test]
fn test_create_table() {
    let conn = Connection::open_in_memory().expect("Failed to open in-mem db");
    assert!(create_table(&conn).is_ok(), "Table creation failed");
}

#[test]
fn test_load_data_from_csv() {
    let conn = Connection::open_in_memory().expect("Failed to open in-mem db");
    create_table(&conn).expect("Table creation failed");

    let file_path = "../data/Spotify_Most_Streamed_Songs.csv";
    assert!(load_data_from_csv(&conn, file_path).is_ok(),
        "Loading CSV data failed");
}

#[test]
fn test_query_create() {
    let conn = Connection::open_in_memory().expect("Failed to open in-mem db");
    create_table(&conn).expect("Table creation failed");
    assert!(query_create(&conn).is_ok(), "Insert query failed");
}

#[test]
fn test_query_read() {
    let conn = Connection::open_in_memory().expect("Failed to open in-mem db");
    create_table(&conn).expect("Table creation failed");
    query_create(&conn).expect("Data insertion failed");
    assert!(query_read(&conn).is_ok(), "Read query failed");
}
