use glob::glob;
use dotenv::dotenv;
use std::{env,io};

pub fn list_file_names() -> io::Result<()> {

    dotenv().ok();

    println!("\nFile List:");

    let wav = env::var("WAV_PATH").expect("Wav File Path Not Found.");
    let m4a = env::var("M4A_PATH").expect("M4A File Path Not Found.");

    let w = glob(&wav).unwrap();
    let m = glob(&m4a).unwrap();

    let dir = w.into_iter().chain(m);

        for paths in dir {
            match paths {
                Ok(path) => {
                    let file_name = path.file_name().unwrap().to_str().unwrap();

                    if file_name.contains("sample") || file_name.contains("Sample") == true {continue;};

                    println!("\nName: {:?}", file_name);
                    println!("\n--------------");
                }
                Err(e) => println!("Error Occured While Trying To List Files:{:?}", e),
            }
        }
    Ok(())
}