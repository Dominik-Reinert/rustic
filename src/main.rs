use std::fs;
use regex::Regex;
use std::vec::Vec;

#[derive(Debug)]
struct FileToImport (String, Vec<String>);

fn main() {
    let file_path = "test-data/hello-world.ts";
    let data = fs::read_to_string(file_path).expect("unable to read file");
    let re = Regex::new(r#"from (?<import>[-'"./A-Za-z0-9]+)"#).unwrap();

    let mut imports: Vec<String> = Vec::new();
    for capture in re.captures_iter(&data) {
        imports.push(String::from(&capture["import"]));
    }
    let fti = FileToImport(file_path.to_string(), imports);
    println!("{:?}", fti);
}
