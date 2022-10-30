use crate::cypher::Cypher;

pub struct AES256 {
    name: String
}

impl AES256 {
    pub fn new(name: String) -> Self {
        Self { name: name}
    }
}

impl Cypher for AES256 {
    fn name(&self) -> &String {
        &self.name
    }
    fn apply(&self) {
        println!("In apply from cypher aes256");
    }
}
