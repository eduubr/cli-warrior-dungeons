use std::fs::File;
use std::io::{BufReader, BufRead, Error};
// use std::path::Path;

// fn teste(path: &str) -> Result<(), Error> {
//     let input = File::open(path)?;
//     let buffered = BufReader::new(input);

//     for line in buffered.lines() {
//         println!("{}", line?);
//     }

//     Ok(())
// }

// fn path_exist(path: &str) {
//     if Path::new(path).exists() {
//         println!("o arquivo/pasta existe!");
//     }else {
//         println!("o arquivo/pasta nao existe!");
//     }
// }

fn run_logo(path: &str) -> Result<(), Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}


fn main() -> Result<(), Error> {
    let path = "logo.txt";
    println!("Hello, world!");
    // path_exist("./logo.txt");
    run_logo(path)
}
