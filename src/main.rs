use serde_json::json;

struct Todo {
    id: u32,
    title: String,
    done: bool,
}

impl Todo {
    fn add(){
        let mut ctr_id = 0;
        Self { id: (ctr_id), title: String::new(), done: false};
        ctr_id += 1;
    }

    fn list(){
        // not implemented
    }

    fn done(&self, id: u32){
        // not implemented
    }

    fn remove(&self, id: u32){
        // not implemented
    }
}

fn main(){
    println!("Hello World!")
}