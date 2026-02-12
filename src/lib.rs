#[allow(warnings)]
mod bindings;

use crate::bindings::exports::component::add::types::{Cat, Guest, Operation};

struct Component;

impl Guest for Component {
    fn hello_world() -> String {
        let res = 10 + 20;
        let add = Operation::Add;
        format!("{:?} : {res}", add)
    }

    fn new_cat() -> Cat {
        Cat {
            name: "Murz".to_string(),
            age: 6,
        }
    }

    fn age() -> u32 {
        10
    }
}

bindings::export!(Component with_types_in bindings);
