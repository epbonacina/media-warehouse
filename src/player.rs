use glob::glob;
use std::fs::File;
use std::io::BufReader;
use std::process::Command;

use mp4;
use serde::{Deserialize, Serialize};
use serde_json;

const MOVIES_DIRECTORY_PATH: &str = "src/movies";

#[derive(Serialize, Deserialize)]
pub struct Movie {
    id: u32,
    name: String,
    duration: String,
}

fn find_movie(id: u32) -> Result<Movie, u32> {
    let movies = get_movies();

    for movie in movies {
        if movie.id == id {
            return Ok(movie);
        }
    }
    Err(id)
}

fn get_movies() -> Vec<Movie> {
    let mut movies = Vec::new();
    let movies_pattern = MOVIES_DIRECTORY_PATH.to_string() + "/*.mp4";
    for (idx, entry) in glob(&movies_pattern)
        .expect("Could not find your movies directory")
        .enumerate()
    {
        match entry {
            Ok(path) => movies.push(Movie {
                id: idx as u32,
                name: path.file_name().unwrap().to_str().unwrap().to_string(),
                duration: get_movie_duration(&format!("{:?}", path.display()).trim_matches('"')),
            }),
            Err(e) => println!("Got an error: {e}"),
        }
    }
    println!("{}", movies.len());
    movies
}

fn get_movie_duration(filepath: &str) -> String {
    println!("Reading file {filepath}");
    let f = File::open(filepath).unwrap();
    let size = f
        .metadata()
        .expect("Could not get size of {filepath}")
        .len();
    let reader = BufReader::new(f);

    let mp4 = mp4::Mp4Reader::read_header(reader, size).unwrap();
    format!("{:.2}", mp4.duration().as_secs_f64()/60f64)
}

pub fn get_movies_as_json() -> String {
    let movies = get_movies();
    serde_json::to_string(&movies).unwrap()
}

pub fn start_movie(id: u32) -> Result<(), u32> {
    let movie = find_movie(id)?;
    let movie_path = format!("{}/{}", MOVIES_DIRECTORY_PATH, movie.name);
    let _ = Command::new("vlc")
        .arg("--play-and-exit")
        .arg("--fullscreen")
        .arg(movie_path)
        .spawn();
    Ok(())
}

pub fn resume_movie() {
    let _ = Command::new("playerctl")
        .arg("--player=vlc")
        .arg("play")
        .output();
}

pub fn pause_movie() {
    let _ = Command::new("playerctl")
        .arg("--player=vlc")
        .arg("pause")
        .output();
}

pub fn quit_movie() {
    let _ = Command::new("playerctl")
        .arg("--player=vlc")
        .arg("stop")
        .output();
}

pub fn rewind_movie(seconds: u16) {
    let offset = format!("{seconds}-");
    let _ = Command::new("playerctl")
        .arg("--player=vlc")
        .arg("position")
        .arg(offset)
        .output();
}

pub fn advance_movie(seconds: u16) {
    println!("Trying to advance {seconds} seconds");
    let offset = format!("{seconds}+");
    let _ = Command::new("playerctl")
        .arg("--player=vlc")
        .arg("position")
        .arg(offset)
        .output();
}
