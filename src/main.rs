use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

mod player;

struct Request {
    method: String,
    endpoint: Vec<String>,
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let request = parse_http_request(&mut stream);

    let endpoint = request.endpoint;
    println!("endpoint: {:?}", endpoint);
    let response = match endpoint[0].as_str() {
        "" => format_ok_response(&player::get_movies_as_json()),
        "play" => {
            if let Ok(_) = player::start_movie(endpoint[1].parse::<u32>().unwrap()) {
                format_ok_response("")
            } else {
                format_error_response("Movie not found")
            }
        },
        "resume" => {
            player::resume_movie();
            format_ok_response("")
        },
        "pause" => {
            println!("Trying to pause");
            player::pause_movie();
            format_ok_response("")
        },
        "rewind" => {
            player::rewind_movie(endpoint[1].parse::<u16>().unwrap());
            format_ok_response("")
        },
        "advance" => {
            player::advance_movie(endpoint[1].parse::<u16>().unwrap());
            format_ok_response("")
        },
        "quit" => {
            player::quit_movie();
            format_ok_response("")
        },
        _ => format_error_response("Unkown command")
    };

    stream.write_all(response.as_bytes()).unwrap();
}


fn parse_http_request(stream: &mut TcpStream) -> Request {
    let request_line = get_http_request_line(stream);
    let parts: Vec<&str> = request_line.split(" ").collect();
    let endpoint: Vec<String>= parts[1]
            .to_string()
            .split("/")
            .map(|s| s.to_string())
            .collect();

    let request = Request {
        method: parts[0].to_string(),
        endpoint: endpoint[1..].to_vec()
    };
    request
}


fn get_http_request_line(stream: &mut TcpStream) -> String {
    let buf_reader = BufReader::new(stream);
    buf_reader.lines().next().unwrap().unwrap()
}


fn format_ok_response(content: &str) -> String {
    format!(
        "HTTP/1.1 200 OK
Content-length: {}
Content-Type: application/json
Access-Control-Allow-Origin: *

{}",
        content.len(),
        content
    )
}


fn format_error_response(content: &str) -> String {
    format!(
        "HTTP/1.1 500 Internal Server Error
Content-length: {}
Content-Type: application/json

{}",
        content.len(),
        content
    )
}
