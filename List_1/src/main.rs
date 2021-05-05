
#[derive(Debug, Clone)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn plus1(l: List<i32>) -> List<i32> {
    match l {
        Cons(i, tail) =>  Cons(i+1, Box::new(plus1(*tail))),
        Nil => Nil
    }
}

fn map<T,U>(l: List<T>, f: fn (T) -> U) -> List<U> {
    match l {
        Cons(e, tail) => Cons(f(e), Box::new(map(*tail, f))),
        Nil => Nil
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("orig {:?}", list);
    println!("+1   {:?}", plus1(list.clone()));
    println!("*2   {:?}", map(list.clone(), |x| x*2));
}
