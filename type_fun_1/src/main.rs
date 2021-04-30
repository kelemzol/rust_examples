pub trait To {
    type ToT;

    fn to(&self) -> Self::ToT;
}

struct Box(i32);

impl To for Box {
    type ToT = i32;

    fn to(&self) -> Self::ToT {
      let &Box(i) = self;
      return i;
    }
}

fn to_<T>(to: &dyn To<ToT = T>) -> &dyn To<ToT = T> {
    return to;
}

fn toto<T: To + To<ToT = TT>,TT>(to: &dyn To<ToT = TT>) -> <T as To>::ToT {
    return to.to();
}

fn main() {
    println!("Hello");
}
