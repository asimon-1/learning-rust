use regex::Regex;
use std::ffi::OsStr;
use std::fs;

fn apply_transform(contents: &String) -> String {
    let re_base =
        Regex::new(r#"<attribute name="(?P<element>.+?)" value="(?P<value>.*?)"/>"#).unwrap();
    let re_clean = Regex::new(r#"(</?\w+)\s(\w+>)"#).unwrap();

    let base = re_base.replace_all(contents, "<$element>$value</$element>");
    return re_clean.replace_all(&base, "$1$2").to_string();
}

fn main() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let display = path.display();
        println!("Checking path {}", display);
        if path.is_dir() {
            println!("Skipping, {} is a directory.", display);
            continue;
        } else {
            match path.extension().and_then(OsStr::to_str) {
                Some(ext) => {
                    if ext.to_lowercase() == "xml" {
                        println!("Converting file: {}", display);
                        let contents = fs::read_to_string(&path)
                            .expect("Something went wrong reading the file");

                        let result = apply_transform(&contents);
                        fs::write(path, &*result).expect("Unable to write to file")
                    } else {
                        println!("Skipping, {} is not an XML file.", display);
                    }
                }
                _ => {
                    println!("Skipping, could not parse extension for {}.", display);
                    continue;
                }
            };
        }
    }
}
