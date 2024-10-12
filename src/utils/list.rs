use glob::glob;
use dotenv::dotenv;
use std::env;
use std::io;

pub fn list_file_names() -> io::Result<()> {

    dotenv().ok();

    let my_env = env::var("THE_PATH").expect("Path Is Set.");

        for entry in glob(&my_env).expect("Files Exist.") {
            match entry {
                Ok(path) => {
                    println!("\nFile List:");
                    println!("\n--------------");
                    println!("\nName: {:?}", path.file_name().unwrap());
                    println!("\n--------------\n")
                }
                Err(e) => println!("{:?}", e),
            }
        }
    Ok(())
}
