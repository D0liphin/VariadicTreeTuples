pub struct Unit<T>(pub T);

impl<T> Unit<T> {
    pub fn with<U>(self, value: U) -> Pair<T, Unit<U>> {
        Pair(self.0, Unit(value))
    }
}

// impl<T> TySeq for Unit<T> {
//     type First = T;
//     type Last = T;
//     type WithoutFirst = ();
//     type WithoutLast = ();

//     fn first(self) -> (Self::First, Self::WithoutFirst) {
//         todo!()
//     }

//     fn last(self) -> (Self::WithoutLast, Self::Last) {
//         todo!()
//     }
// }

pub struct Pair<First, Second>(pub First, pub Second);

impl<First, Second> Pair<First, Second>
where
    Self: TySeq,
{
    pub fn replace<NewValue>(self, value: NewValue) -> (First, Pair<NewValue, Second>) {
        (self.0, Pair(value, self.1))
    }

    pub fn next(self) -> PartialTySeqMap<<Self as TySeq>::WithoutFirst, Unit<<Self as TySeq>::First>> {
        let (head, tail) = self.first();
        PartialTySeqMap {
            src: tail,
            dst: Unit(head),
        }
    }
}

pub trait With<Value>: TySeq + Sized {
    type WithValue;

    fn with(self, value: Value) -> Self::WithValue;
}

impl<First, Second, Third> With<Third> for Pair<First, Unit<Second>> {
    type WithValue = Pair<First, Pair<Second, Unit<Third>>>;

    fn with(self, value: Third) -> Self::WithValue {
        Pair(self.0, Pair(self.1 .0, Unit(value)))
    }
}

impl<First, Second, Tail, Value> With<Value> for Pair<First, Pair<Second, Tail>>
where
    Pair<Second, Tail>: TySeq<First = Second> + With<Value>,
{
    type WithValue = Pair<First, <Pair<Second, Tail> as With<Value>>::WithValue>;

    fn with(self, value: Value) -> Self::WithValue {
        Pair(self.0, self.1.with(value))
    }
}

pub trait TySeq {
    type First;
    type Last;
    type WithoutFirst;
    type WithoutLast;

    fn first(self) -> (Self::First, Self::WithoutFirst);
    fn last(self) -> (Self::WithoutLast, Self::Last);
}

impl<First, Last> TySeq for Pair<First, Unit<Last>> {
    type First = First;
    type Last = Last;
    type WithoutFirst = Unit<Last>;
    type WithoutLast = Unit<First>;

    fn first(self) -> (First, Self::WithoutFirst) {
        (self.0, self.1)
    }

    fn last(self) -> (Self::WithoutLast, Last) {
        (Unit(self.0), self.1 .0)
    }
}

impl<First, Second, Tail, Last> TySeq for Pair<First, Pair<Second, Tail>>
where
    Pair<Second, Tail>: TySeq<First = Second, Last = Last>,
{
    type First = First;
    type Last = Last;
    type WithoutFirst = Pair<Second, Tail>;
    type WithoutLast = Pair<First, <Pair<Second, Tail> as TySeq>::WithoutLast>;

    fn first(self) -> (First, Self::WithoutFirst) {
        (self.0, self.1)
    }

    fn last(self) -> (Self::WithoutLast, Last) {
        let (first, tail) = self.first();
        let (tail_without_last, last) = tail.last();
        (Pair(first, tail_without_last), last)
    }
}

pub struct PartialTySeqMap<Src, Dst> {
    pub src: Src,
    pub dst: Dst,
}

impl<T, Src: TySeq<First = T>, U> PartialTySeqMap<Src, Unit<U>> {
    pub fn next(self) -> PartialTySeqMap<Src::WithoutFirst, Pair<U, Unit<Src::First>>> {
        let (head, tail) = self.src.first();
        PartialTySeqMap {
            src: tail,
            dst: Pair(self.dst.0, Unit(head)),
        }
    }

    pub fn replace<NewValue>(
        self,
        value: NewValue,
    ) -> (
        Src::First,
        PartialTySeqMap<Pair<NewValue, Src::WithoutFirst>, Unit<U>>,
    ) {
        let (head, tail) = self.src.first();
        (
            head,
            PartialTySeqMap {
                src: Pair(value, tail),
                dst: self.dst,
            },
        )
    }
}

impl<T, Src: TySeq<First = T>, Dst: TySeq + With<T>> PartialTySeqMap<Src, Dst> {
    pub fn next(self) -> PartialTySeqMap<<Src as TySeq>::WithoutFirst, <Dst as With<T>>::WithValue> {
        let (src_head, src_tail) = self.src.first();
        PartialTySeqMap {
            src: src_tail,
            dst: self.dst.with(src_head),
        }
    }

    pub fn replace<NewValue>(
        self,
        value: NewValue,
    ) -> (
        <Src as TySeq>::First,
        PartialTySeqMap<Pair<NewValue, <Src as TySeq>::WithoutFirst>, Dst>,
    ) {
        let (head, tail) = self.src.first();
        (
            head,
            PartialTySeqMap {
                src: Pair(value, tail),
                dst: self.dst,
            },
        )
    }
}

impl<T, Dst: TySeq + With<T>> PartialTySeqMap<Unit<T>, Dst> {
    pub fn next(self) -> <Dst as With<T>>::WithValue {
        self.dst.with(self.src.0)
    }

    pub fn replace<NewValue>(self, value: NewValue) -> (T, PartialTySeqMap<Unit<NewValue>, Dst>) {
        (
            self.src.0,
            PartialTySeqMap {
                src: Unit(value),
                dst: self.dst,
            },
        )
    }
}