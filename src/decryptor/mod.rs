use crate::ArgParser;

use crate::selector::select_padder;
use crate::selector::select_cypher;
use crate::selector::select_mode;

pub fn decrypt(arg_parser: &ArgParser) {
    let mode = select_mode(&arg_parser.mode);
    let cypher = select_cypher(&arg_parser.cypher);
    let padder = select_padder(&arg_parser.padder);
    let file_path = &arg_parser.file_path;
    println!("In decrypt -> path: {} mode: {} cypher: {} padder: {}", file_path, mode.name(), cypher.name(), padder.name());
}
