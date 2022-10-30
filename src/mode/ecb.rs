use crate::mode::Mode;

use crate::cypher::Cypher;
use crate::padder::Padder;

pub struct ECB {
    name: String
}


impl ECB {
    pub fn new(name: String) -> Self {
        Self { name: name}
    }
}

impl Mode for ECB {
    fn name(&self) -> &String {
        &self.name
    }
    fn apply(&self, cypher: &Box<dyn Cypher>,
                    padder: &Box<dyn Padder>,
                    file_path: &String,
                    key_path: &String) {
    println!("In apply mode ecb -> file_path: {} key_path: {} cypher: {} padder: {}",
             file_path,
             key_path,
             cypher.name(),
             padder.name());

    }
}
