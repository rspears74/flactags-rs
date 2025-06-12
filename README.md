# flactags-rs

A simple command-line tool written in Rust for batch editing FLAC file tags. It allows you to set, print, and interactively edit common metadata fields (such as artist, album, genre, track number, etc.) for all FLAC files in a directory.

## Features
- Batch edit tags for all FLAC files in a directory
- Set artist, album, album artist, genre, date, and more
- Print tags for all files
- Interactive editing for titles, track numbers, and disc numbers

## Installation

1. Install Rust (if you haven't already):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone this repository and build:
   ```sh
   git clone https://github.com/yourusername/flactags-rs.git
   cd flactags-rs
   cargo build --release
   ```
3. Move the binary onto your `PATH`.

## Usage

Run the tool from the command line:

```sh
flactags [OPTIONS]
```

### Options
- `--print`                 Print tags for all FLAC files
- `--dir <DIR>`             Directory to scan (defaults to current directory)
- `--album <ALBUM>`         Set album tag
- `--artist <ARTIST>`       Set artist tag
- `--album-artist <NAME>`   Set album artist tag
- `--genre <GENRE>`         Set genre tag
- `--date <DATE>`           Set date tag
- `--titles`                Interactively edit titles
- `--track-numbers`         Interactively edit track numbers
- `--disc-numbers`          Interactively edit disc numbers

### Example

Set the artist and album for all FLAC files in the current directory:

```sh
flactags --artist "My Artist" --album "My Album"
```

Print all tags:

```sh
flactags --print
```

Interactively edit track numbers:

```sh
flactags --track-numbers
```

## License

MIT
