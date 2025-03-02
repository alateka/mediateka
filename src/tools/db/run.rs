use diesel::prelude::*;

pub fn connect() -> SqliteConnection {

    SqliteConnection::establish("mediateka.db")
        .unwrap_or_else(|_| panic!("Error connecting to {}", "mediateka.db"))
}