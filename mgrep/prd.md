# mgrep

cli that searches for the pattern in files and current directory.`mgrep PATTERN {optional path} {flags}` also color and links if possible.

## required Behaviour

prints `filename : line_nos`

### flags

- `-i` case-insensitive
- `-c` just count per file
  
  `line_no code blocks......`

### Checklist

- [x] the cli works without breaking
- [x] flags works
- [x] clearly documented
- [x] understood iterators and basic programming in rust
- [x] search core is a single iterator chain + one reused closure, zero manual index counters
- [x] file-list building is its own iterator chain (walk → filter ext → skip target)
- [x] running mgrep inside this workspace doesn't try to read target/
