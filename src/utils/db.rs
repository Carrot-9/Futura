use sqlx::Pool;
use sqlx::mysql::MySql;

use std::env;
use dotenv::dotenv;

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

