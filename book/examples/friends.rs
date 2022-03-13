#![allow(unused_imports)]
#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Friend {
    name: String,
    friends: Vec<Rc<RefCell<Friend>>>,
}

impl Friend {
    fn new(name: &str) -> Self {
        Friend {
            name: name.to_string(),
            friends: vec![],
        }
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Friend::new("Zero Cool")));
    let b = Rc::new(RefCell::new(Friend::new("Acid Burn")));
    let c = Rc::new(RefCell::new(Friend::new("Cereal Killer")));
    
    a.borrow_mut().friends.push(b.clone());
    a.borrow_mut().friends.push(c.clone());
    b.borrow_mut().friends.push(a.clone());
    b.borrow_mut().friends.push(c.clone());
    c.borrow_mut().friends.push(a.clone());
    c.borrow_mut().friends.push(b.clone());
   
    for i in [a, b, c] {
        println!("friends of {}", i.borrow().name);
        for f in i.borrow().friends.iter() {
            println!("- {}", f.borrow().name);
        }
    }
}
