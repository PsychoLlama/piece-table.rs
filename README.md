# Piece Table
This was part of a larger ambition to write my own text editor. Then I got
bored and gave up. But hey, at least I finished the piece table
implementation.

If you're curious, a [piece table](https://darrenburns.net/posts/piece-table/)
is a clever data structure designed to optimize string manipulations and their
affect on memory. But considering you found this repository, you probably know
that already. So here's a disclaimer: I have no idea what I'm doing, and I'm
a terrible Rust programmer. I wouldn't recommend using it.

Anyway, here's the gist:
```rust
use piece_table::Document;

fn main() {
    let mut doc = Document::from("Old title");
    doc.insert(3, " revised");
    doc.delete(&(0..5));
    doc.insert(0, "R");

    assert_eq!(doc.to_string(), "Revised title");
}
```
