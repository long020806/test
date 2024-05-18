enum ConList {
    Cons(i32, Box<ConList>),
    Nil,
}
use std::{ops::Deref, rc::Rc};

use ConList::{Cons, Nil};

struct MyBox<T>(T,T);
impl<T> MyBox<T>{
    pub fn new(x:T,y:T) -> MyBox<T> {
        MyBox(x,y)
    }
}
    
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let b = Cons(5,Box::new(list));
    // let c = Cons(6,Box::new(list));
    let rcList = Rc::new(RcList::Cons(1, Rc::new(RcList::Nil)));

    let b = RcList::Cons(2, Rc::clone(&rcList));
    let c = RcList::Cons(3, Rc::clone(&rcList));

}
