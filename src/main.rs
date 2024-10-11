use glob::glob;
use dotenv::dotenv;
use std::env;

fn list_file_names() {

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

        for entry in glob(&my_env).expect("Files Exist.") {
            match entry {
                Ok(path) => {
                 println!("\nName: {:?}", path.file_name().unwrap());
                }
                Err(e) => println!("{:?}", e),
            }
        }
}

fn main() {
    dotenv().ok();

    list_file_names();

 }

 

 



