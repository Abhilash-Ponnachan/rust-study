use std::fs::File;
use std::io;
use std::io::Read;


use std::env;

// return type is Result<T,E> enum
fn read_from_file(path: &str) -> Result<String, io::Error>{
    let f = File::open(path);
    
    let mut f = match f{
        Ok(file) => file,
        Err(error) => return Err(error)
        // if error return will be variant Err with data 'error'
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        // return variant Ok with data 's' 
        Err(error) => Err(error)
        // if error return will be variant Err with data 'error'
    }
}

fn main() {
    let file = "inputs.txt";
    let pwd = env::current_dir().unwrap();
    println!("Current directory is {:?}", pwd);

    // check the return value and handle as needed
    match read_from_file(file){
        Ok(txt) => println!("File contents => {}", txt),
        Err(error) => println!("Error reading!! => {}", error)
    }
    
}
