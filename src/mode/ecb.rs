use crate::Mode;

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
}
