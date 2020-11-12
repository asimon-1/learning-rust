use std::fs;
use std::path::Path;
use regex::Regex;

fn apply_transform(contents: &String) -> String {
    let re_base = Regex::new(r#"<attribute name="(?P<element>.+?)" value="(?P<value>.*?)"/>"#).unwrap();
    let re_clean = Regex::new(r#"(</?\w+)\s(\w+>)"#).unwrap();

    let base = re_base.replace_all(contents,"<$element>$value</$element>");
    return re_clean.replace_all(&base, "$1$2").to_string()

}

fn main() {
    let filename = "test.XML";
    let path = Path::new(&filename);
    let display = path.display();
    println!("In file {}", display);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    let result = apply_transform(&contents);
    println!("{}", result);
    fs::write(filename, &*result).expect("Unable to write to file")
}
