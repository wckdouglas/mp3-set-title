# MP3 set title #

MP3 files loading into the players (e.g. car audio players), only show the title in their metadata, instead of their file names.

This is a simple program to set title of a mp3 file using it's filename.

## rust ##

The program is written in Rust, and can be installed with `cargo install` (get [started here](https://doc.rust-lang.org/cargo/getting-started/installation.html)).

```
$ cargo install --path .
$ mp3-set-title --help
Set title of mp3 files in a given directory as their file names

Usage: mp3-set-title --mp3-directory <MP3_DIRECTORY>

Options:
  -m, --mp3-directory <MP3_DIRECTORY>  Directory containing all mp3 files
  -h, --help                           Print help
  -V, --version                        Print version
```

## python ##

The prototype was written in python and dependencies are managed by [poetry](https://python-poetry.org/).

```
$ poetry run python python/mp3-set-title.py --help
Usage: mp3-set-title.py [OPTIONS]

  Set title of mp3 files in a given directory as their file names

  assuming the file is named as {album}-{song}.mp3

  :param str mp3_directory: Directory containing all mp3 file :raises
  ValueError: if the given directory does not exist

Options:
  -m, --mp3-directory TEXT  Directory containing all mp3 files  [required]
  --help                    Show this message and exit.
```
