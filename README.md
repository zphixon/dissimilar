Dissimilar: diff library with semantic cleanup
==============================================

This library is a (fork of dtolnay's) port of the Diff component of [Diff Match Patch] to Rust. The diff implementation
is based on [Myers' diff algorithm] but includes some [semantic cleanups] to increase human readability by factoring out
commonalities which are likely to be coincidental.

Diff Match Patch was originally built in 2006 to power Google Docs.

[Diff Match Patch]: https://github.com/google/diff-match-patch
[Myers' diff algorithm]: https://neil.fraser.name/writing/diff/myers.pdf
[semantic cleanups]: https://neil.fraser.name/writing/diff/

```toml
[dependencies]
dissimilar = "1.0"
```

*Compiler support: requires rustc 1.31+*

<br>

## Interface

Here is part of the API. It operates on borrowed strings and the return value of the diff algorithm is a vector of
chunks pointing into slices of those input strings.

```rust
pub enum Chunk<'a> {
    Equal(&'a str),
    Delete(&'a str),
    Insert(&'a str),
}

pub fn diff(text1: &str, text2: &str) -> Vec<Chunk>;
```

<br>

## License

The diff algorithm in this crate was ported to Rust using the Java and C++
implementations found at <https://github.com/google/diff-match-patch> as
reference, and is made available here under the <a href="LICENSE-APACHE">Apache
License, Version 2.0</a> matching the license of the original. This entire
project, including some parts unmodified from upstream and the Rust-specific
modifications introduced in the course of porting the implementation, are
distributed under this Apache license.

Intellectual property that is unique to the Rust implementation is additionally
made available to you dually under the <a href="LICENSE-MIT">MIT license</a>, if
you prefer. This applies to all design choices and implementation choices not
found in the upstream repo.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual Apache and MIT licensed, without any additional terms or conditions.
