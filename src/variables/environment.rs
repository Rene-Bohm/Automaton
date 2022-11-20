use std::sync::{Arc, Mutex};

use super::Signal;


#[derive(Debug, Clone)]
pub struct Environment{

    output_signals: Vec<Signal>,
    input_signals: Vec<Signal>,

}

impl Environment{

    pub fn new(num_of_inputs: usize, num_of_output: usize) -> Self{

        let mut inputs: Vec<Signal> = Vec::with_capacity(num_of_inputs);
        let mut outputs: Vec<Signal> = Vec::with_capacity(num_of_inputs);

        for i in 0..num_of_inputs{
            let name = format!("x{}", i);
            inputs.push(Signal::new(name, false));
        }
        for i in 0..num_of_output{
            let name = format!("y{}", i);
            outputs.push(Signal::new(name, false));
        }

        Environment { output_signals: inputs, input_signals: outputs }

    }

    pub fn print(&self) {

        for i in 0..self.input_signals.len(){
            print!("{:?}, ", self.input_signals[i]);
        }
        println!("");
        for i in 0..self.output_signals.len(){
            print!("{:?}, ", self.output_signals[i]);
        }
    }

}