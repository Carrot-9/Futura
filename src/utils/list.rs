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
                Err(e) => println!("{:?}", e),
            }
        }
    Ok(())
}

