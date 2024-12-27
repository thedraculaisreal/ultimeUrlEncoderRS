# ultimeUrlEncoderRS

What is the ultimate url encoder?
I started this project because i wanted to encode my payloads for sqli, but i couldnt find and websites to encode alphanumeric and special chars all together without getting rid of the new line character, and mushing it all into one clump.

The ultimate url encoder does all of that and only that. It takes a file and url encodes every character accept \n, so your payload.txt doesnt become all jacked up.

install steps:

if cargo and rust is not installed install using this command
1. curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

2. run the install.sh scrip ./install.sh

How to use:

1. urlencoder <path_to_file> <output_file_path> <options>
2. if options is = 1. it encodes all chars, if not = 1 only encodes special chars.
