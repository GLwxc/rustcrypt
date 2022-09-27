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

impl AES256 {
    fn new(name: String) -> Self {
        Self { name: name}
    }
}

impl Cypher for AES256 {
    fn name(&self) -> &String {
        &self.name
    }
}

struct PKCS7 {
    name: String
}

impl PKCS7 {
    fn new(name: String) -> Self {
        Self { name: name}
    }
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

fn main() {
    let mode = select_mode(String::from("ECB"));
    let cypher = select_cypher(String::from("AES256"));
    let padder = select_padder(String::from("PKCS7"));
    println!("mode: {} cypher: {} padder: {}",mode.name(), cypher.name(), padder.name());
}
