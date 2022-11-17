use std::sync::{Arc, Mutex};


pub struct Environment{

    output_variables: Arc<Mutex<Vec<bool>>>,
    input_variables:Arc<Mutex<Vec<bool>>>,

}

impl Environment{

    pub fn get_env(&mut self) -> Self{

        Environment { output_variables: Arc::clone(& self.output_variables), input_variables: Arc::clone(& self.input_variables) }

    }

}