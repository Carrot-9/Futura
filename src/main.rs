mod utils;
use utils::{db, list};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{

    // Loads .env file //
    dotenv::dotenv().ok();

    let pool = db::database().await?;

    // Create 'songs' table if not exists //
    db::create_table(&pool).await?;

    // Inserts filenames and filepaths into 'songs' table //
    db::insert_into_songs(&pool).await?;

    // List all .wav files //
    list::list_file_names()?;

    Ok(())
}
 



