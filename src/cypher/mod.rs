pub mod aes256;

pub trait Cypher {
    fn name(&self) -> &String;
}
