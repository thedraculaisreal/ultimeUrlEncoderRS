//use std::io;
use std::env;
mod encoder;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len != 4 {
	eprintln!("ERROR: <executable> <path_to_file> <output_file_path> <options>");
	return Ok(());
    }
    let input = args[1].clone();
    let output = args[2].clone();
    let options: u8 = args[3].clone().parse().unwrap();
    let mut content = String::new();
    content = encoder::encode_content(input, options);
    encoder::write_content(content, output);
    Ok(())
}
