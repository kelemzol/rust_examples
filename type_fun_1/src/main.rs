use std::fmt::Debug;

pub trait To {
    type ToT;

    fn to(&self) -> Self::ToT;
}

#[derive(Debug)]
pub struct Box(i32);

impl To for Box {
    type ToT = i32;

    fn to(&self) -> Self::ToT {
      let &Box(i) = self;
      return i;
    }
}

// Type of input param not equal type of output
//fn to_<T: To + To<ToT = TT>, TT>(to: &dyn To<ToT = TT>) -> &dyn To<ToT = TT> {

// &dyn To<ToT = TT> not equal T
//fn to_<T: To + To<ToT = TT>, TT>(to: &dyn To<ToT = TT>) -> &T {

// TT not mentioned
//fn to_<T: To + To<ToT = TT>, TT>(to: &T) -> &T {

fn to_<T: To>(to: &T) -> &T {
    return to;
}
// need concrete types at call
// let b__ : i32= toto::<Box,i32>(&b);
//fn toto<T: To + To<ToT = TT>,TT>(to: &dyn To<ToT = TT>) -> <T as To>::ToT {
fn toto<T: To + To<ToT = TT>,TT>(to: &T) -> TT {
    return to.to();
}

fn main() {
    let b = Box(23);
    println!("Hello {:?}", b);
    let b_ = to_(&b);
    println!("Hello {:?}", b_);
    let b__ = toto(&b);
    println!("Hello {:?}", b__);
}
