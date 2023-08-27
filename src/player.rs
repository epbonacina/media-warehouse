use std::fs;
use std::process::Command;

use serde::{Deserialize, Serialize};
use serde_json;

const MOVIES_DESCRIPTOR_PATH: &str = "movies.json";
const MOVIES_DIRECTORY_PATH: &str = "src/movies";

#[derive(Serialize, Deserialize)]
pub struct Movie {
    id: u32,
    name: String,
    description: String,
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
    let file_content = fs::read_to_string(MOVIES_DESCRIPTOR_PATH).unwrap();
    let movies: Vec<Movie> = match serde_json::from_str(&file_content) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };
    movies
}

pub fn get_movies_as_json() -> String {
    let movies = get_movies();
    serde_json::to_string(&movies).unwrap()
}

pub fn start_movie(id: u32) -> Result<(), u32> {
    let movie = find_movie(id)?;
    let movie_path = format!("{}/{}", MOVIES_DIRECTORY_PATH, movie.id);
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
