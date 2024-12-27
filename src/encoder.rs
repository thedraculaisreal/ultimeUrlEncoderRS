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
	if char == ' ' {
	    encoded_content.push_str("%20");
	}
	else if char == '!' {
	    encoded_content.push_str("%21");
	}
	else if char == '"' {
	    encoded_content.push_str("%22");
	}
	else if char == '#' {
	    encoded_content.push_str("%23");
	}
	else if char == '$' {
	    encoded_content.push_str("%24");
	}
	else if char == '%' {
	    encoded_content.push_str("%25");
	}
	else if char == '&' {
	    encoded_content.push_str("%26");
	}
	else if char == '\'' {
	    encoded_content.push_str("%27");
	}
	else if char == '(' {
	    encoded_content.push_str("%28");
	}
	else if char == ')' {
	    encoded_content.push_str("%29");
	}
	else if char == '*' {
	    encoded_content.push_str("%2A");
	}
	else if char == '+' {
	    encoded_content.push_str("%2B");
	}
	else if char == ',' {
	    encoded_content.push_str("%2C");
	}
	else if char == '-' {
	    encoded_content.push_str("%2D");
	}
	else if char == '.' {
	    encoded_content.push_str("%2E");
	}
	else if char == '/' {
	    encoded_content.push_str("%2F");
	}
	else if char == ':' {
	    encoded_content.push_str("%3A");
	}
	else if char == ';' {
	    encoded_content.push_str("%3B");
	}
	else if char == '<' {
	    encoded_content.push_str("%3C");
	}
	else if char == '=' {
	    encoded_content.push_str("%3D");
	}
	else if char == '?' {
	    encoded_content.push_str("%3F");
	}
	else if char == '@' {
	    encoded_content.push_str("%40");
	}
	else if char == '[' {
	    encoded_content.push_str("%5B");
	}
	else if char == '\\' {
	    encoded_content.push_str("%5C");
	}
	else if char == ']' {
	    encoded_content.push_str("%5D");
	}
	else if char == '^' {
	    encoded_content.push_str("%5E");
	}
	else if char == '_' {
	    encoded_content.push_str("%5F");
	}
	else if char == '`' {
	    encoded_content.push_str("%60");
	}
	else if char == '{' {
	    encoded_content.push_str("%7B");
	}
	else if char == '|' {
	    encoded_content.push_str("%7C");
	}
	else if char == '}' {
	    encoded_content.push_str("%7D");
	}
	else if char == '~' {
	    encoded_content.push_str("%7E");
	}
	if options == 1 {
	    if char == 'A' {
		encoded_content.push_str("%41");
	    }
	    else if char == 'B' {
		encoded_content.push_str("%42");
	    }
	    else if char == 'C' {
		encoded_content.push_str("%43");
	    }
	    else if char == 'D' {
		encoded_content.push_str("%44");
	    }
	    else if char == 'E' {
		encoded_content.push_str("%45");
	    }
	    else if char == 'F' {
		encoded_content.push_str("%46");
	    }
	    else if char == 'G' {
		encoded_content.push_str("%47");
	    }
	    else if char == 'H' {
		encoded_content.push_str("%48");
	    }
	    else if char == 'I' {
		encoded_content.push_str("%49");
	    }
	    else if char == 'J' {
		encoded_content.push_str("%4A");
	    }
	    else if char == 'K' {
		encoded_content.push_str("%4B");
	    }
	    else if char == 'L' {
		encoded_content.push_str("%4C");
	    }
	    else if char == 'M' {
		encoded_content.push_str("%4D");
	    }
	    else if char == 'N' {
		encoded_content.push_str("%4E");
	    }
	    else if char == 'O' {
		encoded_content.push_str("%4F");
	    }
	    else if char == 'P' {
		encoded_content.push_str("%50");
	    }
	    else if char == 'Q' {
		encoded_content.push_str("%51");
	    }
	    else if char == 'R' {
		encoded_content.push_str("%52");
	    }
	    else if char == 'S' {
		encoded_content.push_str("%53");
	    }
	    else if char == 'T' {
		encoded_content.push_str("%54");
	    }
	    else if char == 'U' {
		encoded_content.push_str("%55");
	    }
	    else if char == 'V' {
		encoded_content.push_str("%56");
	    }
	    else if char == 'W' {
		encoded_content.push_str("%57");
	    }
	    else if char == 'X' {
		encoded_content.push_str("%58");
	    }
	    else if char == 'Y' {
		encoded_content.push_str("%59");
	    }
	    else if char == 'Z' {
		encoded_content.push_str("%5A");
	    }
	    else if char == 'a' {
		encoded_content.push_str("%61");
	    }
	    else if char == 'b' {
		encoded_content.push_str("%62");
	    }
	    else if char == 'c' {
		encoded_content.push_str("%63");
	    }
	    else if char == 'd' {
		encoded_content.push_str("%64");
	    }
	    else if char == 'e' {
		encoded_content.push_str("%65");
	    }
	    else if char == 'f' {
		encoded_content.push_str("%66");
	    }
	    else if char == 'g' {
		encoded_content.push_str("%67");
	    }
	    else if char == 'h' {
		encoded_content.push_str("%68");
	    }
	    else if char == 'i' {
		encoded_content.push_str("%69");
	    }
	    else if char == 'j' {
		encoded_content.push_str("%6A");
	    }
	    else if char == 'k' {
		encoded_content.push_str("%6B");
	    }
	    else if char == 'l' {
		encoded_content.push_str("%6C");
	    }
	    else if char == 'm' {
		encoded_content.push_str("%6D");
	    }
	    else if char == 'n' {
		encoded_content.push_str("%6E");
	    }
	    else if char == 'o' {
		encoded_content.push_str("%6F");
	    }
	    else if char == 'p' {
		encoded_content.push_str("%70");
	    }
	    else if char == 'q' {
		encoded_content.push_str("%71");
	    }
	    else if char == 'r' {
		encoded_content.push_str("%72");
	    }
	    else if char == 's' {
		encoded_content.push_str("%73");
	    }
	    else if char == 't' {
		encoded_content.push_str("%74");
	    }
	    else if char == 'u' {
		encoded_content.push_str("%75");
	    }
	    else if char == 'v' {
		encoded_content.push_str("%76");
	    }
	    else if char == 'w' {
		encoded_content.push_str("%77");
	    }
	    else if char == 'x' {
		encoded_content.push_str("%78");
	    }
	    else if char == 'y' {
		encoded_content.push_str("%79");
	    }
	    else if char == 'z' {
		encoded_content.push_str("%7A");
	    }
	    else if char == '0' {
		encoded_content.push_str("%30");
	    }
	    else if char == '1' {
		encoded_content.push_str("%31");
	    }
	    else if char == '2' {
		encoded_content.push_str("%32");
	    }
	    else if char == '3' {
		encoded_content.push_str("%33");
	    }
	    else if char == '4' {
		encoded_content.push_str("%34");
	    }
	    else if char == '5' {
		encoded_content.push_str("%35");
	    }
	    else if char == '6' {
		encoded_content.push_str("%36");
	    }
	    else if char == '7' {
		encoded_content.push_str("%37");
	    }
	    else if char == '8' {
		encoded_content.push_str("%38");
	    }
	    else if char == '9' {
		encoded_content.push_str("%39");
	    }
	}
	encoded_content.push(char);
    }
    encoded_content
}
