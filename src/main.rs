mod cypher;
mod padder;
mod mode;

use cypher::Cypher;
use mode::Mode;
use padder::Padder;

use crate::mode::ecb::ECB;
use crate::cypher::aes256::AES256;
use crate::padder::pkcs7::PKCS7;

pub fn select_mode(mode: String) -> Box<dyn Mode> {
    match mode.as_str() {
        "ECB" => Box::new(ECB::new(mode)),
        _     => panic!("error: {} is unknown.", mode),

    }
}

pub fn select_cypher(cypher: String) -> Box<dyn Cypher> {
    match cypher.as_str() {
        "AES256" => Box::new(AES256::new(cypher)),
        _     => panic!("error: {} is unknown.", cypher),

    }
}


pub fn select_padder(padder: String) -> Box<dyn Padder> {
    match padder.as_str() {
        "PKCS7" => Box::new(PKCS7::new(padder)),
        _     => panic!("error: {} is unknown.", padder),

    }
}

pub fn encrypt(path: &String, mode: &Box<dyn Mode>, cypher: &Box<dyn Cypher>, padder: &Box<dyn Padder>) {
    println!("In encrypt -> path: {} mode: {} cypher: {} padder: {}", path, mode.name(), cypher.name(), padder.name());
}


pub fn decrypt(path: &String, mode: &Box<dyn Mode>, cypher: &Box<dyn Cypher>, padder: &Box<dyn Padder>) {
    println!("In decrypt -> path: {} mode: {} cypher: {} padder: {}", path, mode.name(), cypher.name(), padder.name());
}

fn main() {
    let mode = select_mode(String::from("ECB"));
    let cypher = select_cypher(String::from("AES256"));
    let padder = select_padder(String::from("PKCS7"));
    let path = String::from("/path/to/something");
    encrypt(&path, &mode, &cypher, &padder);
    decrypt(&path, &mode, &cypher, &padder);
}
