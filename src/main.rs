// Pushes song and sample files to my database 'sopranodb'

// When building this website, I started writing the automation code first,
// which is why this src is not inside of its own folder.
// If I were to try to move it into a seperate folder, my environmental variables would break,
// so I've decided to just leave it as is.

// When looking through this github repo, assume that everything in the root is in it's own seperate, labeled directory

mod utils;
use utils::{db, list};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{

    // Loads .env file //
    dotenv::dotenv().ok();
    
   let db = run_db().await;
   
   match db {
        Ok(_) => println!("\nDatabase Ran Without Issue."),
        Err(e) => eprintln!("Error Occured While Running DB: {}",e),
   };

    // List all .wav files //
    list::list_file_names()?;

    Ok(())
}

async fn run_db() -> Result<(), sqlx::Error> {

      // Initialize database connection //
      let pool = db::database().await?;

     // Create 'songs' table if not exists //
     db::create_tables(&pool).await?;

     // Inserts filenames and filepaths into 'songs' table //
     db::insert_into_songs(&pool).await?;

     // Removes duplicate columns in table 'songs'
     db::remove_duplicates(&pool).await?;
 
     Ok(())
}
 



