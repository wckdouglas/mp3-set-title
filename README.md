# MP3 set title #

MP3 files loading into music players (e.g. car audio players) only show the title from their metadata, instead of their file names. This can create confusion when we compile a library of these mp3 files in a hard drive and connect it to the (car) player.

This is a simple program to set titles of mp3 files using their filenames. We assume the files are named as `{album}-{song title}.mp3`, and the final mp3 files will have:
- title: `{album}-{song title}`
- album: `{album}`

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

A prototype is written in python and dependencies are managed by [poetry](https://python-poetry.org/).

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
