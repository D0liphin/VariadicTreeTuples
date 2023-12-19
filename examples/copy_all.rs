#![feature(fmt_internals)]
use recursive_type::*;
use std::fmt;

trait ForAllCopy {
    type Output;

    fn copy_all(self) -> (Self::Output, Self::Output);
}

impl<Head: Copy, Tail: ForAllCopy> ForAllCopy for Pair<Head, Tail> {
    type Output = Pair<Head, Tail::Output>;

    fn copy_all(self) -> (Self::Output, Self::Output) {
        let (lhs, rhs) = self.1.copy_all();
        (Pair(self.0, lhs), Pair(self.0, rhs))
    }
}

impl<T: Copy> ForAllCopy for Unit<T> {
    type Output = T;
    fn copy_all(self) -> (Self::Output, Self::Output) {
        (self.0, self.0)
    }
}

trait ForAllDebug {
    fn debug_all<'a, 'b>(self, f: &'a mut fmt::Formatter<'b>) -> fmt::Result;
}

impl<Head, Tail, Next> ForAllDebug for Pair<Head, Tail>
where
    Head: fmt::Debug,
    Next: fmt::Debug,
    Tail: ForAllDebug + TySeq<First = Next>,
{
    fn debug_all<'a, 'b>(self, f: &'a mut fmt::Formatter<'b>) -> fmt::Result {
        match write!(f, "{:?} -> ", self.0) {
            Ok(..) => Ok(self.1.debug_all(f)?),
            Err(e) => Err(e),
        }
    }
}

impl<First: fmt::Debug, Second: fmt::Debug> ForAllDebug for Pair<First, Unit<Second>> {
    fn debug_all<'a, 'b>(self, f: &'a mut fmt::Formatter<'b>) -> fmt::Result {
        write!(f, "{:?} -> {:?}", self.0, self.1 .0)
    }
}

fn main() {
    let seq = Unit(1i32)
        .with(2u32)
        .with(3u8)
        .with("string")
        .with((1i32, 2u64));
    let (seq1, seq2) = seq.copy_all();

    // let mut s = String::new();
    // let mut formatter = fmt::Formatter::new(&mut s);
    // let _ = seq1.debug_all(&mut formatter);
    // println!("{s}");
}
