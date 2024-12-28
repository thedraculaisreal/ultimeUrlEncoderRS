use std::fs;
use std::fs::*;
use std::io::prelude::*;

// writes encoded content to a new output file.
pub fn write_content(content: String, output: String){
    File::create(&output).expect("Failed to create file");
    fs::write(&output, content).expect("Failed to write content.");
    println!("Done encoding file...")
}

// urlencodes all chars, including alphanumeric.
pub fn encode_content(input: String, options: u8) -> String {
    let mut file = File::open(input)
	.expect("File not found");
    let mut contents = String::new();
    let mut encoded_content = String::new();
    file.read_to_string(&mut contents).expect("Failed to read txt into buffer");
    for char in contents.chars() {
	match char {
	    ' ' => encoded_content.push_str("%20"),
	    '!' => encoded_content.push_str("%21"),
	    '"' => encoded_content.push_str("%22"),
	    '#' => encoded_content.push_str("%23"),
	    '$' => encoded_content.push_str("%24"),
	    '%' => encoded_content.push_str("%25"),
	    '&' => encoded_content.push_str("%26"),
	    '\'' => encoded_content.push_str("%27"),
	    '(' => encoded_content.push_str("%28"),
	    ')' => encoded_content.push_str("%29"),
	    '*' => encoded_content.push_str("%2A"),
	    '+' => encoded_content.push_str("%2B"),
	    ',' => encoded_content.push_str("%2C"),
	    '-' => encoded_content.push_str("%2D"),
	    '.' => encoded_content.push_str("%2E"),
	    '/' => encoded_content.push_str("%2F"),
	    ':' => encoded_content.push_str("%3A"),
	    ';' => encoded_content.push_str("%3B"),
	    '<' => encoded_content.push_str("%3C"),
	    '=' => encoded_content.push_str("%3D"),
	    '?' => encoded_content.push_str("%3F"),
	    '@' => encoded_content.push_str("%40"),
	    '[' => encoded_content.push_str("%5B"),
	    '\\' => encoded_content.push_str("%5C"),
	    ']' => encoded_content.push_str("%5D"),
	    '^' => encoded_content.push_str("%5E"),
	    '_' => encoded_content.push_str("%5F"),
	    '`' => encoded_content.push_str("%60"),
	    '{' => encoded_content.push_str("%7B"),
	    '|' => encoded_content.push_str("%7C"),
	    '}' => encoded_content.push_str("%7D"),
	    '~' => encoded_content.push_str("%7E"),
	    _ => encoded_content.push(char)
	}
    }
    encoded_content
}
