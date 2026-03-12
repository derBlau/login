mod errors;
mod utils;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = PathBuf::from("./database/userdata.db");

    if fs::exists(db_path)? {
        println!("found");
    } else {
        println!("not found");
    }
    Ok(())
}
