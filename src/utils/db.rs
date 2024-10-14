use sqlx::Pool;
use sqlx::mysql::MySql;

use std::env;
use dotenv::dotenv;
use glob::glob;

struct Songs {
    id: i32,
    name: Option<String>,
    file_path: Option<String>
}

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

pub async fn _get_file_metadata(_conn: Pool<MySql>) -> Result<(), sqlx::Error> {

    dotenv().ok();

    let mut n:i32 = 0;
    increment(&mut n);

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

    for entry in glob(&my_env).expect("Files Exist.") {
        let entry = entry.expect("Unable To Get Entry");
        let file_path = entry.display();
        let file_name =  entry.file_name().unwrap();
        let file_id = n;
        let combined = format!("{:?} {:?} {:?}", file_id, file_name, file_path);
        println!("{:?}", combined);
        };

    Ok(())
    }





