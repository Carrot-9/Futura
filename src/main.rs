mod utils;
use utils::{db,list};

fn main() -> Result<(), std::io::Error> {
  
    // List all .wav files //

    list::list_file_names()?;

    // Set Up Database //

   let conn = db::connect();

   match conn {
    Ok(_) => println!("\nDatabase Connected Without Issue...\n"),
    Err(e) => println!("Error Occured Trying To Connect To Database: {}", e),
   }
   
    Ok(())

}
 



