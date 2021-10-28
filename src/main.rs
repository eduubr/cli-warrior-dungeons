// use std::fs::File;
// use std::io::{BufReader, BufRead, Error};
// use std::path::Path;
// use std::fs::File;
// use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;


fn main() {
    let path = "logo.txt";
    let end_game: bool = false;
    check_save();
    loop {
        if end_game { break }
        run_logo(path);
    }
}


fn check_save() {
    let g_save: bool = path_exist("./save.txt");
    let logo: bool = path_exist("./logo.txt");
    if !(g_save && logo) {
        println!("algum arquivo esta faltando!");
    }
}

fn path_exist(path: &str) -> bool {
    Path::new(path).exists()

    //     println!("o arquivo/pasta existe!");
    // }else {
    //     println!("o arquivo/pasta nao existe!");
    // }
}

fn run_logo(path: &str) {
    let mut input = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
