pub mod ecb;

use std::io;

use crate::cypher::Cypher;
use crate::padder::Padder;

pub trait Mode {
    fn name(&self) -> &String;
    fn apply(&self, cypher: &Box<dyn Cypher>,
                    padder: &Box<dyn Padder>,
                    file_path: &String,
                    key_path: &String) -> io::Result<()>;
}
