pub trait Padder {
    fn name(&self) -> &String;
}

pub trait Mode {
    fn name(&self) -> &String;
}

pub trait Cypher {
    fn name(&self) -> &String;
}

struct AES256 {
    name: String
}

impl Cypher for AES256 {
    fn name(&self) -> &String {
        &self.name
    }
}

struct PKCS7 {
    name: String
}

impl Padder for PKCS7 {
    fn name(&self) -> &String {
        &self.name
    }
}

struct ECB {
    name: String
}


impl ECB {
    fn new(name: String) -> Self {
        Self { name: name}
    }
}

impl Mode for ECB {
    fn name(&self) -> &String {
        &self.name
    }
}

pub fn select_mode(mode: String) -> Box<dyn Mode> {
    match mode.as_str() {
        "ECB" => Box::new(ECB::new(mode)),
        _     => panic!("error: {} is unknown.", mode),

    }
}

fn main() {
    let mode = select_mode(String::from("ECB"));
    println!("{}",mode.name());
}
