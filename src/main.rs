// use std::env;
use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
    let filename = "test.XML";
    let path = Path::new(&filename);
    let display = path.display();
    println!("In file {}", display);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    let re = Regex::new(r#"<attribute name="(?P<element>.+?)" value="(?P<value>.*?)".*>"#).unwrap();
    let result = re.replace_all(&contents, "<$element>$value</$element>");
    println!("{}", result);
    fs::write(filename, &*result).expect("Unable to write to file")
}
