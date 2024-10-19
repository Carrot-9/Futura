use sqlx::MySqlPool;

use std::env;
use dotenv::dotenv;
use glob::glob;

pub struct Connection {
   pub conn: MySqlPool,
}

pub fn get_database_path() -> String {
    dotenv().ok();
    let db_path = env::var("DATABASE_URL").expect("Database Path Exists.");
    return db_path;
}

#[tokio::main]
 pub async fn connect() -> Result<MySqlPool, sqlx::Error>{

let database_path = get_database_path();

// Creating Instance of Connection struct //
let db_connection: Connection = Connection {
    conn: sqlx::MySqlPool::connect(&database_path).await?,
};

sqlx::query(
    r#"
CREATE TABLE IF NOT EXISTS songs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);"# 
    )
    .execute(&db_connection.conn)
    .await?;
 
 Ok(db_connection.conn)
}

pub async fn _insert_into_songs(db_connection: Connection) -> Result<MySqlPool, sqlx::Error> {

    dotenv().ok();

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

    for entry in glob(&my_env).expect("Files Exist.") {
        let entry = entry.expect("Unable To Get Entry");
        let file_name  =  entry.file_name().unwrap().to_str();
        let file_path  = entry.display().to_string();

        sqlx::query("INSERT INTO songs(name, file_path) VALUES(?, ?)")
            .bind(file_name)
            .bind(file_path)
            .execute(&db_connection.conn)
            .await?;
    };

    Ok(db_connection.conn)

    }



