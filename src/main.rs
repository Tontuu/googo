use tiny_http::{Server, Response, Request, Method, StatusCode, Header};
use std::fs::File;
use std::path::Path;
use std::io;

fn get_content_type(filepath: &str) -> &'static str {
    let path = Path::new(filepath);

    let extension = match path.extension() {
        None    => return "text/plain",
        Some(e) => e,
    };

    match extension.to_str().unwrap() {
        "gif"  => "image/gif",
        "jpg"  => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png"  => "image/png",
        "svg"  => "image/svg+xml",
        "pdf"  => "application/pdf",
        "css"  => "text/css",
        "htm"  => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt"  => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8",
    }
}

fn serve_static_file(request: Request, mut filepath: &str) -> io::Result<()> {
    if filepath.starts_with("/") {
        filepath = &filepath[1..];
    }

    let content_type = get_content_type(filepath);

    let content_type_header = Header::from_bytes("Content-Type", content_type)
        .expect("Garbage on content-type headers!!!!");

    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: Could not serve file {filepath}: {err}");
            return request.respond(Response::from_string("404").with_status_code(StatusCode(404)));
        }
    };

    println!("RESPONSE: {filepath} with `{content_type}`");
    request.respond(Response::from_file(file).with_header(content_type_header))
}

fn serve_request(mut request: Request) -> io::Result<()> {
    println!("INFO: received request. method: {:?}, url: {:?}", request.method(), request.url());

    match (request.method(), request.url()) {
        (Method::Get, "/") | (Method::Get, "/index.html") => {
            return serve_static_file(request, "index.html");
        },

        (Method::Post, "/query") => {
            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).unwrap();
            println!("POST: {} | {:?}", request.url(), content);
            return serve_static_file(request, "index.html");
        },

        (Method::Get, _) => {
            let filepath = request.url().to_string();
            return serve_static_file(request, &filepath);
        },

        _ => {
            return request.respond(Response::from_string("404").with_status_code(StatusCode(404)));
        }
    }
}

fn main() -> Result<(), ()>{
    let server = Server::http("0.0.0.0:6969").unwrap();
    let port = server.server_addr().to_ip().unwrap().port();
    println!("Listening at port: {port}");


    for request in server.incoming_requests() {
        serve_request(request).map_err(|err| {
            eprintln!("ERROR: Could not serve the response: {err}");
        }).ok();
    }

    Err(())
}
