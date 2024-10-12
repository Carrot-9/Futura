mod utils;
use utils::{db,list};

fn main() -> Result<(), std::io::Error> {
  
    // List all .wav files //

    list::list_file_names()?;

    // Set Up Database //

    let db = db::db();

    match db {
        Ok(_) => println!("Database Connected Without Issue."),
        Err(e) => println!("Error Has Occured: {}", e),
    }

    Ok(())

}
 



