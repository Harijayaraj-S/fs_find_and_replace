use std::{
    fs::{self, File},
    io::Read,
};

fn main() {
    let from = "old";
    let to = "new";

    let mut file = File::open("FILE_PATH").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let updated_content = contents.replace(from, to);

    fs::write("FILE_PATH", updated_content).unwrap();
}
