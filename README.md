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
- fd-find
- skim
- ripgrep
- git2
- tantivy

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
