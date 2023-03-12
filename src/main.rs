use clap::Parser;
use glob::glob;
use id3::{Tag, TagLike};
use log::info;
use std::path::{Path, PathBuf};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory containing all mp3 files
    #[arg(short, long, value_parser=check_folder_exists)]
    mp3_directory: String,
}

/// check if a file exists
///
/// # Arguments
/// - file_path: a file path to test for it's existence
///
/// # Returns
/// - Err if no
fn check_folder_exists(file_path: &str) -> Result<String, String> {
    match Path::new(file_path).exists() {
        true => Ok(file_path.to_string()),
        false => Err(format!("{} is not a valid directory", file_path)),
    }
}

/// Mutate the Tags in a give mp3 file
///
/// # Arguments
/// - mp3_file: the file path to a mp3 file
/// - title: the title of to be set for the song
/// - album: the album to be set for the song
fn mutate_mp3_file_metadata(mp3_file: &Path, title: &str, album: &str) -> Result<(), String> {
    let mp3_file_string = check_folder_exists(mp3_file.to_str().ok_or("mp3 file is not a str")?)?;
    info!("Mutating file: {}", mp3_file_string);
    let mut tag: Tag = Tag::read_from_path(&mp3_file_string).map_err(|e| e.to_string())?;
    tag.set_title(title);
    tag.set_album(album);
    tag.write_to_path(&mp3_file_string, tag.version())
        .map_err(|e| e.to_string())?;
    info!(
        "Mutating file: {} with title: [{}] and album: [{}]",
        &mp3_file_string, title, album
    );
    Ok(())
}

/// For a given directory, find all the mp3 files and set the title as it's filename,
/// assuming the file is named as {album}-{song}.mp3
///
/// # Arguments
/// - mp3_directory: the file path to the directory containing all the mp3 files
fn run(mp3_directory: String) -> Result<(), String> {
    let file_pattern: String = format!("{}/*.mp3", mp3_directory);
    let mp3_file_list = glob(&file_pattern).map_err(|e| e.to_string())?;
    for mp3_file in mp3_file_list {
        let file_path: PathBuf = mp3_file.map_err(|e| e.to_string())?;
        let filename: &str = file_path
            .file_name()
            .ok_or("file does not have basename")?
            .to_str()
            .ok_or("basename is not a str")?;
        let title: &str = filename
            .strip_suffix(".mp3")
            .ok_or("does the file ends with .mp3?")?;
        let album: Vec<&str> = filename.split('-').collect();
        mutate_mp3_file_metadata(&file_path, title, album[0])?;
    }
    Ok(())
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Args::parse();
    let result = run(args.mp3_directory);
    match result {
        Ok(_) => (),
        Err(err_string) => println!("{}", err_string),
    };
}
