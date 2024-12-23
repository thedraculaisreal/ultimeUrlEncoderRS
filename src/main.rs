//use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("/home/schwarztoter/projects/coding/rust/ultimeUrlEncoderRS/Auth_Bypass.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // passing a ref mutable char, so we can change the chars to url encoded versions of themselves.
    for char in contents.chars() {
	if char == ' ' {
	    print!("%20");
	}
	else if char == '!' {
	    print!("%21");
	}
	else if char == '"' {
	    print!("%22");
	}
	else if char == '#' {
	    print!("%23");
	}
	else if char == '$' {
	    print!("%24");
	}
	else if char == '%' {
	    print!("%25");
	}
	else if char == '&' {
	    print!("%26");
	}
	else if char == '\'' {
	    print!("%27");
	}
	else if char == '(' {
	    print!("%28");
	}
	else if char == ')' {
	    print!("%29");
	}
	else if char == '*' {
	    print!("%2A");
	}
	if char == ' ' {
	    print!("%20");
	}
	else if char == '!' {
	    print!("%21");
	}
	else if char == '"' {
	    print!("%22");
	}
	else if char == '#' {
	    print!("%23");
	}
	else if char == '$' {
	    print!("%24");
	}
	else if char == '%' {
	    print!("%25");
	}
	else if char == '&' {
	    print!("%26");
	}
	else if char == '\'' {
	    print!("%27");
	}
	else if char == '(' {
	    print!("%28");
	}
	else if char == ')' {
	    print!("%29");
	}
	else if char == '*' {
	    print!("%2A");
	}
	else if char == '+' {
	    print!("%2B");
	}
	else if char == ',' {
	    print!("%2C");
	}
	else if char == '-' {
	    print!("%2D");
	}
	else if char == '.' {
	    print!("%2E");
	}
	else if char == '/' {
	    print!("%2F");
	}
	else if char == '0' {
	    print!("%30");
	}
	else if char == '1' {
	    print!("%31");
	}
	else if char == '2' {
	    print!("%32");
	}
	else if char == '3' {
	    print!("%33");
	}
	else if char == '4' {
	    print!("%34");
	}
	else if char == '5' {
	    print!("%35");
	}
	else if char == '6' {
	    print!("%36");
	}
	else if char == '7' {
	    print!("%37");
	}
	else if char == '8' {
	    print!("%38");
	}
	else if char == '9' {
	    print!("%39");
	}
	else if char == ':' {
	    print!("%3A");
	}
	else if char == ';' {
	    print!("%3B");
	}
	else if char == '<' {
	    print!("%3C");
	}
	else if char == '=' {
	    print!("%3D");
	}
	else if char == '?' {
	    print!("%3F");
	}
	else if char == '@' {
	    print!("%40");
	}
	else if char == 'A' {
	    print!("%41");
	}
	else if char == 'B' {
	    print!("%42");
	}
	else if char == 'C' {
	    print!("%43");
	}
	else if char == 'D' {
	    print!("%44");
	}
	else if char == 'E' {
	    print!("%45");
	}
	else if char == 'F' {
	    print!("%46");
	}
	else if char == 'G' {
	    print!("%47");
	}
	else if char == 'H' {
	    print!("%48");
	}
	else if char == 'I' {
	    print!("%49");
	}
	else if char == 'J' {
	    print!("%4A");
	}
	else if char == 'K' {
	    print!("%4B");
	}
	else if char == 'L' {
	    print!("%4C");
	}
	else if char == 'M' {
	    print!("%4D");
	}
	else if char == 'N' {
	    print!("%4E");
	}
	else if char == 'O' {
	    print!("%4F");
	}
	else if char == 'P' {
	    print!("%50");
	}
	else if char == 'Q' {
	    print!("%51");
	}
	else if char == 'R' {
	    print!("%52");
	}
	else if char == 'S' {
	    print!("%53");
	}
	else if char == 'T' {
	    print!("%54");
	}
	else if char == 'U' {
	    print!("%55");
	}
	else if char == 'V' {
	    print!("%56");
	}
	else if char == 'W' {
	    print!("%57");
	}
	else if char == 'X' {
	    print!("%58");
	}
	else if char == 'Y' {
	    print!("%59");
	}
	else if char == 'Z' {
	    print!("%5A");
	}
	else if char == '[' {
	    print!("%5B");
	}
	else if char == '\\' {
	    print!("%5C");
	}
	else if char == ']' {
	    print!("%5D");
	}
	else if char == '^' {
	    print!("%5E");
	}
	else if char == '_' {
	    print!("%5F");
	}
	else if char == '`' {
	    print!("%60");
	}
	else if char == 'a' {
	    print!("%61");
	}
	else if char == 'b' {
	    print!("%62");
	}
	else if char == 'c' {
	    print!("%63");
	}
	else if char == 'd' {
	    print!("%64");
	}
	else if char == 'e' {
	    print!("%65");
	}
	else if char == 'f' {
	    print!("%66");
	}
	else if char == 'g' {
	    print!("%67");
	}
	else if char == 'h' {
	    print!("%68");
	}
	else if char == 'i' {
	    print!("%69");
	}
	else if char == 'j' {
	    print!("%6A");
	}
	else if char == 'k' {
	    print!("%6B");
	}
	else if char == 'l' {
	    print!("%6C");
	}
	else if char == 'm' {
	    print!("%6D");
	}
	else if char == 'n' {
	    print!("%6E");
	}
	else if char == 'o' {
	    print!("%6F");
	}
	else if char == 'p' {
	    print!("%70");
	}
	else if char == 'q' {
	    print!("%71");
	}
	else if char == 'r' {
	    print!("%72");
	}
	else if char == 's' {
	    print!("%73");
	}
	else if char == 't' {
	    print!("%74");
	}
	else if char == 'u' {
	    print!("%75");
	}
	else if char == 'v' {
	    print!("%76");
	}
	else if char == 'w' {
	    print!("%77");
	}
	else if char == 'x' {
	    print!("%78");
	}
	else if char == 'y' {
	    print!("%79");
	}
	else if char == 'z' {
	    print!("%7A");
	}
	else if char == '{' {
	    print!("%7B");
	}
	else if char == '|' {
	    print!("%7C");
	}
	else if char == '}' {
	    print!("%7D");
	}
	else if char == '~' {
	    print!("%7E");
	}
	else {
	    print!("{char}");
	}
    }
    Ok(())
}
