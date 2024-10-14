use sqlx::Pool;
use sqlx::mysql::MySql;

use std::env;
use dotenv::dotenv;
use glob::glob;

fn increment( i: &mut i32) {
    *i += 1;
}

#[tokio::main]
 pub async fn connect() -> Result<Pool<MySql>, sqlx::Error>{

    dotenv().ok();

let database_url = env::var("DATABASE_URL").unwrap();

let conn = sqlx::MySqlPool::connect(&database_url).await?;

sqlx::query(
    r#"
CREATE TABLE IF NOT EXISTS songs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);"# 
    )
    .execute(&conn)
    .await?;
 
 Ok(conn)
}

pub async fn _insert_into_songs(conn: Pool<MySql>) -> Result<(), sqlx::Error> {

    dotenv().ok();

    // n holds increment value of i //
    let mut n:i32 = 0;
    increment(&mut n);

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

    for entry in glob(&my_env).expect("Files Exist.") {
        let entry = entry.expect("Unable To Get Entry");
        let file_id = n;
        let file_name  =  entry.file_name().unwrap().to_str();
        let file_path  = entry.display().to_string();

        sqlx::query("INSERT INTO songs(id, name, file_path) VALUES(?, ?, ?)")
            .bind(file_id)
            .bind(file_name)
            .bind(file_path)
            .execute(&conn)
            .await?;
    };

    Ok(())

    }





