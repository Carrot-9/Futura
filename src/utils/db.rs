use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql,Pool};
use sqlx::MySqlPool;

use std::env;
use glob::glob;

pub async fn database() -> Result<Pool<MySql>, sqlx::Error> {
    let database_path = env::var("DATABASE_URL").expect("Database Path Exists.");
    let pool: Pool<MySql> = MySqlPoolOptions::new() 
    .max_connections(5)
    .connect(&database_path)
    .await?;

    Ok(pool)
}

 pub async fn create_table(pool: &MySqlPool) -> Result<(), sqlx::Error>{

sqlx::query(
    r#"
CREATE TABLE IF NOT EXISTS songs (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    file_path VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, file_path)
);"# 
    )
    .execute(pool)
    .await?;
 
 Ok(())
}

pub async fn insert_into_songs(pool: &MySqlPool) -> Result<(), sqlx::Error> {

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

    for entry in glob(&my_env).expect("Files Exist.") {
        match entry {
            Ok(path) => {
                let file_name  =  path.file_name().unwrap().to_str();
                let file_path  = path.display().to_string();

                sqlx::query("INSERT INTO songs(name, file_path) VALUES(?, ?)")
                    .bind(file_name)
                    .bind(file_path)
                    .execute(pool)
                    .await?;
            }
            Err(e) => eprintln!("Error Occured While Trying To Insert Into Table 'songs': {}", e),
        }
    };
    Ok(())
}




