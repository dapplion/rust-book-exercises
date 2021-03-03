#[derive(Debug)]
enum List {
    Cons(RefCell<i32>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::{borrow::BorrowMut, cell::RefCell};

// b (3) ->
//      a (5) -> y (8) -> z (Nil)
// c (4) ->
fn main() {
    let z = Nil;
    let y = Cons(RefCell::new(8), Rc::new(z));
    let a = Rc::new(Cons(RefCell::new(5), Rc::new(y)));
    let b = Cons(RefCell::new(3), Rc::clone(&a));
    let c = Cons(RefCell::new(4), Rc::clone(&a));

    print_con_list(&c);
    print_con_list(&b);
}

fn print_con_list(list: &List) {
    match list {
        List::Cons(val, l) => {
            println!("{}", val.borrow());
            print_con_list(l);
        }
        List::Nil => println!("Nil"),
    }
}
