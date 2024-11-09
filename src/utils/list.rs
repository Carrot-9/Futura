/* Despite the fact that this list will almost always be discarded via main.sh, 
I still thinks it's useful enough to keep in the codebase
*/

use glob::glob;
use dotenv::dotenv;
use std::{env,io};

pub fn list_file_names() -> io::Result<()> {

    dotenv().ok();

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

    println!("\nFile List:");

        for entry in glob(&my_env).expect("Files Exist.") {
            match entry {
                Ok(path) => {
                    println!("\nName: {:?}", path.file_name().unwrap());
                    println!("\n--------------");
                }
                Err(e) => println!("Error Occured While Trying To List Files:{:?}", e),
            }
        }
    Ok(())
}