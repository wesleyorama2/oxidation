use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub pass: String,
    pub data: String,
}

pub fn create_dbconn() -> rusqlite::Connection {
    let conn_result = Connection::open("oxidation.db");

    match conn_result {
        Ok(connection) => connection,
        Err(error) => panic!("Problem connecting to sqlite db: {:?}", error),
    }
}

pub async fn get_user(conn: &rusqlite::Connection, name: &String) -> Result<Vec<User>> {
    let mut user_results = Vec::new();
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, pass TEXT, data BLOB)", (),)?;
    let mut stmt = conn.prepare("SELECT id, name, pass, data FROM users WHERE name=:name;")?;
    let user_iter = stmt.query_map(&[(":name", name.to_string().as_str())], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            pass: row.get(2)?,
            data: row.get(3)?,
        })
    })?;

    for user in user_iter {
        match user {
            Ok(user) => user_results.push(user),
            Err(error) => return Err(error),
        }
    }

    Ok(user_results)
}

pub async fn create_user(
    conn: &rusqlite::Connection,
    name: &String,
    pass: &String,
    data: &String,
) -> Result<usize> {
    // Probably dont want to run this on every user creation, but I dont expect many people will create more than one user
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, pass TEXT, data BLOB)", (),)?;
    conn.execute(
        "INSERT INTO users (name, pass, data) VALUES (?1, ?2, ?3)",
        params![name, pass, data],
    )
}
