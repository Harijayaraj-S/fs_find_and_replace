use std::{
    fs::{self, File},
    io::Read,
};

fn main() {

    let find_word = "old";
    let replace_word = "new";

    let mut file = File::open("FILE_PATH").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut words_vec: Vec<&str> = contents.split(' ').collect();
    for index in 0..words_vec.len() - 1 {
        if words_vec[index] == find_word {
            words_vec[index] = replace_word;
        }
    }

    fs::write("FILE_PATH", words_vec.join(" ")).unwrap();
}
