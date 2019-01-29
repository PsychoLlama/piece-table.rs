# Motto
A vim-esque editor written in Rust and rendered in the terminal.

## Advantages
- More performant
- Better syntax highlighting (probably through
  [`syntect`](https://crates.io/crates/syntect))
  - With meaningful tokens, you can build smarter plugins and motions
- Plugins written in a modern language
  - Most usefully: capable of sharing 3rd party libraries
  - Plugins become testable
- Autoformatter & fixer integrations
- Linting integrations
- Language server protocol integration
- Better support for embedded programs (e.g. terminals)
- Support for real-time text collaboration
- Standalone stateful editor backend supporting multiple frontends
- Editorconfig integration

In addition, since the editor is built with Rust, it can easily leverage incredibly powerful tools like:
- [fd-find](https://crates.io/crates/fd-find)
- [skim](https://crates.io/crates/skim)
- [ripgrep](https://crates.io/crates/ripgrep)
- [git2](https://crates.io/crates/git2)
- [tantivy](https://crates.io/crates/tantivy)

and doesn't need to pollute command mode with language features, since it's all just rust (and SIGNIFICANTLY easier for me to implement). With luck, during plugin development, people will contribute more Rust language parsers and code intelligence to the OSS community. Win!

## Bare minimums
- Buffers (just the data structure)
- Buffer mutations (transactional)
- File reading
- Buffer views
- Simple CLI interface
- Insert mode
- Line numbers
- Line wrapping

## Compelling demo
- Command mode
- File writing
- Registers
- Normal mode
- Smart indentation
- Multiple views
- View management
- View resizing
- Syntax highlighting (`syntect`)
- View zooming
- File browser
- Visual mode
- In-file search (ripgrep)

## Impressive
- Simple plugin system
- File type detection
- Autoformatting
- LSP integration
- Generic completion mechanism
- Linter integration
- User-defined mappings
- surround.vim plugin
- Configurable status line
- Branching undo/redo
- Tabs (call them screens)
- Marks
- Themeing

## OMG
- Plugin installer
- `s/pat/value` command with real-time preview
- `g/pat/cmd`
- Reload buffer with sudo (relatable factor)
- Some form of proper git integration (through a plugin?)
- Macros (plenty of space for improvement)
- Real-time collaboration plugin

## Peripheral
- Comment autoformatting ala `gq`
- Conceal

## Stances
Features I have no intention of adding

- Vimscript

## Data structure
Inserting text into the middle of a string is a highly innefficient process since everything after the insert has to be moved by the corresponding byte offset. Vectors have to do something similar. While it's relatively cheap for small files, larger files can have a seriously noticeable lag, especially when you're inserting on every keystroke.

Motto doesn't do that. Instead, it uses an optimized data structure called a [piece table](https://darrenburns.net/posts/piece-table/) to more efficiently manage arbitrary buffer insertions. Here's the high-level:

When you read a new file, a new `SourceText` structure is created. The file's contents are loaded into `original` and _never_ directly modified. Instead, we maintain some metadata that describes how to merge the document's original state with our edits. We do this with the `segments` vector.

Each `Segment` (or "slice", if you prefer) is a logical pointer into one of the `original` or `insertions` buffers. Segments indicate where a slice begins and how long it is. When you first read the file, the `segments` vector will have a single `Segment` pointing to the entire `original` buffer. When you eventually go to write the file back to disk, you'll concatenate every slice into a new string and write the result.

In a sense, the original file is the template and each segment an operation.

Deleting text in a buffer is a matter of breaking that text segment into two parts, one pointing to the slice before the deleted text, the other slicing text after it. Since neither segment points to the deleted text, it won't be included the next time you derive document state.

Inserting text is a matter of appending your new string to the `insertions` buffer (note: this operation is _always_ an append), then adding a new `Segment` pointing to the newly added text. If the insertion happened in the middle of another segment, that segment has to be split into two parts, with the insertion added in the middle.

And that's how writes work. On the other side, there are reads. How do you read an arbitrary line, or count the number of lines in a document?

That's where line number caching comes in. When you first load the file, find the byte offset for every newline character and save it into a B-tree set with the `original` buffer. Since it's immutable, you never need to update that cache.

The `insertions` buffer works much the same, but you'll add to the line break set as you insert text.

Now, whenever you create a segment, do a `BTreeSet` range query over your byte
slice. Index each result into a `HashMap` where the keys are line numbers and
the values are byte offsets. Now, to find a given line number, iterate through
the `segments` array and increment a line number count until you find the
right segment.

```rust
struct Segment {
  lines: HashMap<usize, usize>,
  byte_length: usize,
  byte_offset: usize,
  is_new: bool,
}

struct IndexedString {
  linebreaks: BTreeSet<usize>,
  string: String,
}

struct SourceText {
  original_text: IndexedString,
  insertions: IndexedString,
  segments: Vec<Segment>,
}
```
