use sqlx::MySqlPool;

use std::env;
use dotenv::dotenv;


#[tokio::main]
 pub async fn db() -> Result<MySqlPool, sqlx::Error>{

    dotenv().ok();

let database_url = env::var("DATABASE_URL").unwrap();

let conn = sqlx::MySqlPool::connect(&database_url).await?;

 Ok(conn)
}