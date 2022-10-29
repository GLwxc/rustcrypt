mod argparser;
mod encryptor;
mod decryptor;
mod selector;
mod cypher;
mod padder;
mod mode;

use argparser::ArgParser;

use crate::decryptor::decrypt;
use crate::encryptor::encrypt;


fn main() {
    let arg_parser = ArgParser::new();
    match arg_parser.action.as_str() {
        "enc" => encrypt(&arg_parser),
        "dec" => decrypt(&arg_parser),
        _ => panic!("error: {} is unknow.", arg_parser.action),
    }
}
