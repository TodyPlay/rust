use std::fs::File;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = read_file()?;

    let mut str = String::new();

    file.read_to_string(&mut str);

    println!("{}", str);

    Ok(())
}

//传播错误
fn read_file() -> Result<File, Box<dyn Error>> {
    let file = File::open("Cargo.lock")?;
    Ok(file)
}

fn read_files() -> Result<File, Box<dyn Error>> {
    let result = File::open("Cargo.lock");

    match result {
        Err(e) => Err(Box::new(e)),
        Ok(ok) => Ok(ok),
    }
}

