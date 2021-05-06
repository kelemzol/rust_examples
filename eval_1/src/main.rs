
#[derive(Debug, PartialEq)]
enum Expr {
    IntLit(i32),
    BoolLit(bool),
    Sum(Box<Expr>, Box<Expr>),
    And (Box<Expr>, Box<Expr>),
    If (Box<Expr>, Box<Expr>, Box<Expr>),
}

use crate::Expr::{IntLit, BoolLit, Sum, And, If};

fn eval(expr: Expr) -> Option<Expr> {
    match expr {
        IntLit(i) => Some(IntLit(i)),
        BoolLit(b) => Some(BoolLit(b)),
        Sum(a, b) => match (eval(*a), eval(*b)) {
            (Some(IntLit(a)), Some(IntLit(b))) => Some(IntLit(a+b)),
            _ => None
        }
        And(a, b) => match (eval(*a), eval(*b)) {
            (Some(Expr::BoolLit(a)), Some(BoolLit(b))) => Some(BoolLit(a&&b)),
            _ => None
        }
        If(c, t, e) => match eval(*c) {
            Some(BoolLit(true)) => eval(*t),
            Some(BoolLit(false)) => eval(*e),
            _ => None
        }
    }
}

use std::ops;
impl ops::Add for Expr {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Sum(Box::new(self), Box::new(_rhs))
    }
}

impl ops::BitAnd for Expr {
    type Output = Self;

    fn bitand(self, _rhs: Self) -> Self {
        And(Box::new(self), Box::new(_rhs))
    }
}

use std::convert::From;
impl From<i32> for Expr {
    fn from(i: i32) -> Self {
        IntLit(i)
    }
}

impl From<bool> for Expr {
    fn from(b: bool) -> Self {
        BoolLit(b)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::convert::From;
    use crate::Expr::{IntLit, BoolLit, Sum, And, If};
    use crate::eval;

    #[test]
    fn test_int_lit() {
        assert_eq!(Some(IntLit(1)), eval(IntLit(1)));
    }

    #[test]
    fn test_bool_lit() {
        assert_eq!(Some(BoolLit(true)), eval(BoolLit(true)));
    }

    #[test]
    fn test_sum() {
        assert_eq!(Some(IntLit(3)), eval(Sum(Box::new(IntLit(1)), Box::new(IntLit(2)))));
    }

    #[test]
    fn test_and() {
        assert_eq!(Some(BoolLit(true)), eval(And(Box::new(BoolLit(true)),
                                                 Box::new(BoolLit(true)))));
    }

    #[test]
    fn test_if() {
        assert_eq!(Some(IntLit(2)), eval(If(Box::new(BoolLit(false)),
                                            Box::new(IntLit(1)), Box::new(IntLit(2)))));
    }

    #[test]
    fn test_if_diff_type() {
        assert_eq!(Some(IntLit(2)), eval(If(Box::new(BoolLit(false)),
                                            Box::new(BoolLit(true)), Box::new(IntLit(2)))));
    }

    #[test]
    fn test_embedded_sum() {
        // cannot infer type // not bijective type function?
        // assert_eq!(Some(From::from(3)), eval(From::from(1) + From::from(2)));
        assert_eq!(Some(From::from(3)), eval(IntLit(1) + From::from(2)));
    }

    #[test]
    fn test_embedded_and() {
        assert_eq!(Some(From::from(true)), eval(BoolLit(true) & BoolLit(true)));
    }
}
