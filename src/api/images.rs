use std::{
    fs::File,
};


pub mod v1 {
    use super::*;

    #[get("/images/v1/named?<img>")]
    pub fn named(img: String) -> std::io::Result<File> {
        File::open("data/icon.jpg")
    }
}