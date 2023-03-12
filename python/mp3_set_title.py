import logging
from pathlib import Path
import click

from mutagen.easyid3 import EasyID3

logging.basicConfig(level=logging.INFO)


def mod_title(mp3_file: Path):
    logging.info("Mutating file: %s", mp3_file)
    title = mp3_file.name.removesuffix(".mp3")
    album = title.split("-")[0]
    audio = EasyID3(mp3_file)
    audio["title"] = title
    audio["album"] = album
    audio.save()
    logging.info("Mutated file: %s with title: [%s], album [%s]", mp3_file, title, album)


@click.command()
@click.option("-m", "--mp3-directory", required=True, help="Directory containing all mp3 files")
def main(mp3_directory: str):
    mp3_path = Path(mp3_directory)
    if not mp3_path.is_dir():
        raise ValueError(f"{mp3_path} is not a valid directory")

    for mp3_file in mp3_path.glob("*.mp3"):
        mod_title(mp3_file)


if __name__ == "__main__":
    main()
