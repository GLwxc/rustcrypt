use crate::padder::Padder;

pub struct PKCS7 {
    name: String
}

impl PKCS7 {
    pub fn new(name: String) -> Self {
        Self { name: name}
    }
}

impl Padder for PKCS7 {
    fn name(&self) -> &String {
        &self.name
    }
    fn apply(&self) {
        println!("In apply from padder pkcs7");
    }
}
