# mgrep

A simpler version of grep tool written in rust using iterators and closure functionalities.

## Command

`cargo run -- {PATTERN} {Optional paths} {flags}`

## Architecture

- The `search crate` uses `walkdir` crate to travers the file and directories and also used to ignore hidden and generated files.
- `fs::read_to_string` reads the files and provides the content in string format so `match_lines` can search through them.
- The path is `ARC` so multiple cloning of path is not expensive.

## Flags

| Flags | Description |
| ------ | ----------- |
| -i | ignore_case so pattern is case insensitive |
| -c | count_mode just shows file and number of pattern occurances |

A fun learning project :)
