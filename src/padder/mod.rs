pub mod pkcs7;

pub trait Padder {
    fn name(&self) -> &String;
    fn apply(&self);
}
