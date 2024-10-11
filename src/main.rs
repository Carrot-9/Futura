use glob::glob;
use dotenv::dotenv;
use std::env;

use sqlx::mysql::MySqlPoolOptions;

fn list_file_names() {

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

        for entry in glob(&my_env).expect("Files Exist.") {
            match entry {
                Ok(path) => {
                 println!("\nName: {:?}", path.file_name().unwrap());
                 println!("----------------------------------------")
                }
                Err(e) => println!("{:?}", e),
            }
        }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    // List File Names 

    dotenv().ok();

    list_file_names();

    // Setup Database Connection

    let my_database_env = env::var("DATABASE_URL").expect("Database Connected.");

   let _pool = MySqlPoolOptions::new()
    .max_connections(5)
    .connect(&my_database_env).await;

    Ok(())
 }

 

 



