# cryptopals

My solutions to [the cryptopals crypto challenges](https://cryptopals.com/).

This is also a practice for rust, so the code does not represent good practice.

## Why this project is abandoned?

I set out to learn some interesting and useful new programming language,
meanwhile solving the crypto challenges. Rust seemed like a good choice.
However, after about 6 days' rust experience (3 weekends), I decided to abandon
rust and also this project (at least for now). I don't think rust has made a
good trade-off between "safety" and convenience for me, and it's also evolving
kind of too slowly. For example, I encountered issues like
[#479](https://github.com/rust-lang/rfcs/issues/479),
[#22741](https://github.com/rust-lang/rust/issues/27741) and
[#29625](https://github.com/rust-lang/rust/issues/29625), which are almost 1
year old and still unresolved, thus really frustrating for new users like me.
Specifically, I also found the following defects (or features):

1. [Index](https://doc.rust-lang.org/std/ops/trait.Index.html) must return a
   reference, which is really a pain for custom math code. (So I can only
   implement an `.index` method but can not utilize the `[]` operator)
2. Rust does not allow impl non-local traits on bare local type parameters (for
   example, `impl<T:S> ::std::ops::Add<i32> for T` for some local trait `S`). So
   I can not write several structs and impl some std trait for them in a unified
   way.
3. Rust lacks auto return type deduction like `decltype(auto)` in C++14,
   forcing my to write a `U8IterRefRm` in [bytearray.rs](src/bytearray.rs).
   Luckily, a relevant RFC for `impl Trait` has recently been
   [merged](https://github.com/rust-lang/rfcs/pull/1522#issuecomment-228895459)
   (but it still can not solve my case).
4. It's sometimes difficult to manage lifetime; see my
   [SO question](http://stackoverflow.com/q/40449512/1184354).

In summary, rust concepts like explicit ownership, lifetime parameters and
enforced move-assignment are interesting and inspiring. The document is also
well-organized and effective to search/browse. However I think some of the
design considerations are overemphasized and get in the way of writing concise
code or code that even *compiles*. In these days I spent most of my time
fighting with the compiler or skimming unresolved RFCs/issues, and had little
time for actually thinking about the problem that I want to solve. It is fun to
learn, but not productive enough for me in real-world projects.

So, I still can not find an alternative for the C++/python combination. And I am
going to continue on cryptopals with python :)
