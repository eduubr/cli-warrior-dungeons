use std::{fmt,path::Path,fs::metadata};

pub mod teste{

    fn path_exist(path: &str) {
        if Path::new(path).exists() {
            println!("o arquivo/pasta existe!");
        }else {
            println!("o arquivo/pasta existe!");
        }
    }
}

// pub fn exists(&self) -> bool {
//     metadata(self).is_ok()
// }