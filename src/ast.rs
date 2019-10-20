use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub struct Header {
    comments: Vec<Comment>,
}

#[derive(Debug)]
pub struct Comment {
    line: usize,
    text: String,
}

// impl Debug for Expr {
//     fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
//         use self::Expr::*;
//         match *self {
//             Number(n) => write!(fmt, "{:?}", n),
//             Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
//             Error => write!(fmt, "error"),
//         }
//     }
// }
