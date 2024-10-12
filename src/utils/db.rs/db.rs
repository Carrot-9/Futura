use sqlx::mysql::MySqlPoolOptions;

use std::env;
use dotenv::dotenv;


#[tokio::main]
 pub async fn db() -> Result<(), sqlx::Error> {

    dotenv().ok();

let my_database_env = env::var("DATABASE_URL").expect("Database Connected.");

let _pool = MySqlPoolOptions::new()
 .max_connections(5)
 .connect(&my_database_env).await?;

 println!("Pool Is Connected.");

 Ok(())
}