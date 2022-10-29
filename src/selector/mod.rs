use crate::cypher::Cypher;
use crate::mode::Mode;
use crate::padder::Padder;

use crate::mode::ecb::ECB;
use crate::cypher::aes256::AES256;
use crate::padder::pkcs7::PKCS7;

pub fn select_mode(mode: String) -> Box<dyn Mode> {
    match mode.as_str() {
        "ecb" => Box::new(ECB::new(mode)),
        _     => panic!("error: {} is unknown.", mode),

    }
}

pub fn select_cypher(cypher: String) -> Box<dyn Cypher> {
    match cypher.as_str() {
        "aes256" => Box::new(AES256::new(cypher)),
        _     => panic!("error: {} is unknown.", cypher),

    }
}


pub fn select_padder(padder: String) -> Box<dyn Padder> {
    match padder.as_str() {
        "pkcs7" => Box::new(PKCS7::new(padder)),
        _     => panic!("error: {} is unknown.", padder),

    }
}
