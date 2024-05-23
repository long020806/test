use rusqlite::Error;
use rusqlite::{Connection};
use std::io;
use std::io::BufRead;
const CREATE_TABLE: &str = "CREATE TABLE books 
                            (id INTEGER PRIMARY KEY, 
                            title TEXT NOT NULL, 
                            author TEXT NOT NULL, 
                            year INTEGER NOT NULL)";
const DROP_TABLE: &str = "DROP TABLE IF EXISTS books";
#[derive(Debug)]
struct Book {
    id: u32,
    title: String,
    author: String,
    year: u16,
}
fn main() {
    let conn = Connection::open("./books").unwrap();
    init_database(&conn);
    insert(&conn);
    query(&conn);
}

fn init_database(conn: &Connection) {

    match conn.execute(CREATE_TABLE,[]) {
        Ok(_) => {
            println!("创建表格成功")
        },
        Err(_) => {
            println!("创建失败")
        },
    };
}

fn insert(conn: &Connection) {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {

        let elems = line.unwrap();
        let elems: Vec<&str> = elems.split(",").collect();
    println!("{:?}",elems);

        if elems.len() == 4 {
            let _ = conn.execute(
                "INSERT INTO books (id, title, author, year) VALUES (?1, 
?2, ?3, ?4)",
                &[&elems[0], &elems[1], &elems[2], &elems[3]],
            );
        }
    }
}

fn query(conn: &Connection) {
    let mut stmt = conn
        .prepare(
            "SELECT id, title, author, year FROM books WHERE year >= 
?1",
        )
        .unwrap();
    let movie_iter = stmt
        .query_map(&[&1], |row| Ok(Book {
            id: row.get(0).unwrap(),
            title: row.get(1).unwrap(),
            author: row.get(2).unwrap(),
            year: row.get(3).unwrap(),
        }))
        .unwrap();
    for movie in movie_iter.filter_map(extract_ok) {
        println!("Found book {:?}", movie);
    }
}

fn extract_ok(p: Result<Book, Error>) -> Option<Book> {
    if p.is_ok() {
        Some(p.unwrap())
    } else {
        None
    }
}
