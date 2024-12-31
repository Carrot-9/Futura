use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql,Pool};
use sqlx::MySqlPool;

use glob::glob;
use std::env;

pub async fn database() -> Result<Pool<MySql>, sqlx::Error> {

    let database_path: String = env::var("DATABASE_URL").expect("Database Path Does Not Exist.");
    let pool: Pool<MySql> = MySqlPoolOptions::new() 
    .max_connections(5)
    .connect(&database_path)
    .await?;

    Ok(pool)
}

 pub async fn create_tables(pool: &MySqlPool) -> Result<(), sqlx::Error>{

    sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS songs (
        id INT AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        file_path VARCHAR(255) NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );"# 
        )
        .execute(pool)
        .await?;

    sqlx::query("CREATE TABLE IF NOT EXISTS samples LIKE songs")
        .execute(pool)
        .await?;

 Ok(())

}

pub async fn insert_into_songs(pool: &MySqlPool) -> Result<(), sqlx::Error> {

    let wav = env::var("WAV_PATH").expect("Wav File Path Not Found.");
    let m4a = env::var("M4A_PATH").expect("M4A File Path Not Found.");

    let w = glob(&wav).unwrap();
    let m = glob(&m4a).unwrap();

    let dir = w.into_iter().chain(m);

    for paths in dir {
        match paths {
            Ok(path) => {
                let file_name  =  path.file_name().unwrap().to_str().unwrap();
                let file_path = path.to_str().unwrap();
                let table_name;

                if file_name.contains("sample") || file_name.contains("Sample") {
                    table_name = "samples";
                } else {
                    table_name = "songs";
                }

                let query = format!("INSERT INTO {}(name, file_path) VALUES(?,?)", table_name);

                sqlx::query(&query)
                    .bind(file_name)
                    .bind(file_path)
                    .execute(pool)
                    .await?;
            }
            Err(e) => eprintln!("Error Occured While Trying To Query Database: {}", e),
        }
    };
    Ok(())
}

pub async fn remove_duplicates(pool: &MySqlPool) -> Result<(), sqlx::Error> {

    sqlx::query(r#"
     DELETE FROM songs
     WHERE id NOT IN (
     SELECT * FROM(
     SELECT MIN(id) 
     FROM songs
     GROUP BY name, file_path
     ) AS alias)
;"#
    )
    .execute(pool)
    .await?;

    sqlx::query(r#"
     DELETE FROM samples 
     WHERE id NOT IN (
     SELECT * FROM(
     SELECT MIN(id) 
     FROM samples
     GROUP BY name, file_path
     ) AS alias)
;"#
    )
    .execute(pool)
    .await?;
 
Ok(())
}
