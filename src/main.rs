mod utils;
use utils::{db,list};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
  
    // List all .wav files //

    list::list_file_names()?;

    // Set Up Database //

    let database = db::connect();

    match database {
        Ok(_) => println!("\nDatabase Connected Without Issue."),
        Err(e) => println!("Error Occured Trying To Connect To Database: {} ", e),
    }

   // Inserts .wav files into mysql database //

    Ok(())

}
 



