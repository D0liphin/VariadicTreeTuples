use recursive_type::*;

trait ForAllCopy {}

impl<Head, Tail, Next> ForAllCopy for Pair<Head, Tail>
where
    Head: Copy,
    Next: Copy,
    Tail: ForAllCopy + TySeq<First = Next>,
{
}

impl<First: Copy, Second: Copy> ForAllCopy for Pair<First, Unit<Second>> {}

fn main() {}
