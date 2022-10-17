use std::{env, fs};

pub fn read_file(folder: &str, name: &str) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd.join("src").join(folder).join(format!("{}.txt", name));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}