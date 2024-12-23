//use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("/home/schwarztoter/projects/coding/rust/ultimeUrlEncoderRS/Auth_Bypass.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // passing a ref mutable char, so we can change the chars to url encoded versions of themselves.
    for char in contents.chars() {
	if char == '\'' {
	    print!("%27");
	}
	else {
	    print!("{char}");
	}
    }
    Ok(())
}
