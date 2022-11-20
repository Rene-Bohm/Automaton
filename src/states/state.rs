use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use crate::variables::Environment;

#[derive(Debug, Clone)]
pub struct State{

    environment: Arc<Mutex<Environment>>,

}

impl State {
    
    pub fn print(&self) {
        self.environment.lock().unwrap().print();
    } 

}



#[cfg(test)]
mod test{
    use std::{sync::{Mutex, Arc}, thread};
    use crate::variables::Environment;

    use super::State;


    #[test]
    fn get(){

        let mut env = Arc::new(Mutex::new(Environment::new(4,2)));
        let refer = Arc::clone(&env);

        let state = thread::spawn(move || {
        
            let state = State{environment: refer};

            state.print();
        
        });

        state.join().unwrap();

    }


}