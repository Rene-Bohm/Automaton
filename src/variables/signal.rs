#[derive(Debug, Clone)]
pub struct Signal{

    name: String,
    state: bool,
    
}

impl Signal {
    
    pub fn new(name: String, value: bool)-> Self{
        Signal { name: name, state: value }
    }

}