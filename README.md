# Mp3 set title #

This is a simple program to set title of a mp3 file as it's filename

## rust ##

```bash
$ cargo install --path .
$ mp3_set_title --help
Set title of mp3 files in a given directory as their file names

Usage: mp3_set_title --mp3-directory <MP3_DIRECTORY>

Options:
  -m, --mp3-directory <MP3_DIRECTORY>  Directory containing all mp3 files
  -h, --help                           Print help
  -V, --version                        Print version
```

## python ##

```bash
$ poetry run python mp3_set_title.py --help
Usage: mp3_set_title.py [OPTIONS]

Options:
  -m, --mp3-directory TEXT  Directory containing all mp3 files  [required]
  --help                    Show this message and exit.
```
