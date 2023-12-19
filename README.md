# Recursive Types

We don't have variadic types in Rust, though we do have something that is basically good enough 
(macros). However, we can kind of still do variadic types in Rust.

I try and avoid doing this kind of thing in actuality though, because I think there are not that 
many cases where we actually want to increase compile time by something ungodly for the sake of 
a "cleaner function". Just use macros, to be honest.

This came about when I created this API (which still required simpler recursive types)

```rust
Assignment
    .with(&mut dst_1, src_1)
    .with(&mut dst_2, src_2)
    .with(&mut dst_3, src_3)
    .with(&mut dst_4, src_4)
    .assign_all();
```

This is quite easy to implement, but the issue is, we cannot create `src_3` in-place if it relies on
a reference to `dst_1` or `dst_2`. As a result, I wanted this API

```rust
Assignment
    .with_value(src_1)
    .with_value(src_2)
    .with_value(src_3)
    .with_value(src_4)
    .with_dst(&mut dst_1)
    .with_dst(&mut dst_2)
    .with_dst(&mut dst_3)
    .with_dst(&mut dst_4)
    .assign_all();
```

This is much harder to implement. We can implement a version that assigns backwards easily, with a
nested tuple structure, but I don't like that.

So the library makes use of two types `Pair` and `Unit`. A `TySeq` is implement for any nest `Pair`
that ends in a `Unit`.

This allows us to implement a `.last()` function on any `TySeq` by recursive calls to the tail of 
each sequence. Kind of fun.

```rust
impl TySeq for Pair<First, Unit<Last>> {
    type WithoutLast = Unit<First>;
}

impl TySeq for Pair<First, Pair<Second, Tail>> {
    type WithoutLast = Pair<First, <Pair<Second, Tail> as TySeq>::WithoutLast>;
}
```

So, for example, `Pair<A, Pair<B, Pair<C, Unit<D>>>>` implements `TySeq` because of the following 
deduction. Note that I just use `(A, B)` for `Pair<A, B>` and `(A)` for `Unit<A>`

```rust
Type            | WithoutLast
(A,             | (A, ...     
    (B,         |     (B, ...
        (C,     |         (C)))   
            (D) | 
        )       |
    )           |
)               |
```

This way, we can also implement a recursive last function that just unwraps the tuple and replaces
the tail with itself without the last element. This might sound like it's a lot of work, but it 
compiles to the machine code that you would hope for, in my tests.

## Doing things on all the types in the sequence...

You have to implement a new 'forall' trait (I think) that asserts that each item implements some
other trait. This is done like this:

```rust
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
```

You can't take the sequence as reference and do anything with it sadly. The structure needs to
be destructurable (moved around a lot). So we can't do this. We can however, define methods on 
it based on traits that all of the elements implement. Like above. This might seem like a lot of 
work, but it's quite obvious boilerplate. Here, I strip away the function implementations and
some Rust bloat.

```rs
trait ForAllCopy;
impl ForAllCopy for (impl Copy);
impl ForAllCopy for (impl Copy, impl ForAllCopy);
```

Really simple now! Kind of like declarative programming...