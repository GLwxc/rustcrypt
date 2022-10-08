pub mod ecb;

pub trait Mode {
    fn name(&self) -> &String;
}
