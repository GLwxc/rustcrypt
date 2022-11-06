use crate::mode::Mode;

use crate::cypher::Cypher;
use crate::padder::Padder;

use std::io::Read;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use std::io;
use std::io::prelude::*;


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
                    key_path: &String) -> io::Result<()> {
    println!("In apply mode ecb -> file_path: {} key_path: {} cypher: {} padder: {}",
             file_path,
             key_path,
             cypher.name(),
             padder.name());
    // read file
    let f = File::open(file_path.to_string())?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    
    reader.read_to_end(&mut buffer)?;
    // padder.apply(&mut buffer);
    //
    // for block in buffer
    //      let encrypted_block = cypher.apply(block);
    //
    // write the last block to a file

    let mut obuffer = BufWriter::new(File::create(file_path.to_string() + ".enc")?);
    obuffer.write_all(&buffer)?;
    obuffer.flush()?;
    Ok(())
    }
}
