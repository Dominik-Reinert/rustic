use std::fs;
use regex::Regex;

fn main() {
    let file_path = "test-data/hello-world.ts";
    let _import_regex = Regex::new(r"import.*").unwrap();
    let data = fs::read_to_string(file_path).expect("unable to read file");


    println!("{}", &data);
    println!("");

    let re = Regex::new(r#"from (?<import>[-'"./A-Za-z0-9]+)"#).unwrap();
    
    for capture in re.captures_iter(&data) {
        println!("{}", &capture["import"]);
    }
}
