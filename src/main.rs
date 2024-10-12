mod utils;
use utils::{db,list};

fn main() {
    // List all .wav files //

  let _ = list::list_file_names();

    // Set Up Database //

    let _ = db::db();
}
 



