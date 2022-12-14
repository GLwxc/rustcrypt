use crate::ArgParser;

use crate::selector::select_padder;
use crate::selector::select_cypher;
use crate::selector::select_mode;

pub fn encrypt(arg_parser: &ArgParser) {
    let mode = select_mode(&arg_parser.mode);
    let cypher = select_cypher(&arg_parser.cypher);
    let padder = select_padder(&arg_parser.padder);
    let file_path = &arg_parser.file_path;
    let key_path = &arg_parser.key_path;
    mode.apply(&cypher,
               &padder,
               file_path,
               key_path);
}
