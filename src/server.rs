use tiny_http::{Server, Response, Request, Method, StatusCode, Header};
use std::fs::File;
use std::path::Path;
use std::io;

pub fn get_content_type(filepath: &str) -> &'static str {
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

pub fn serve_404(request: Request) -> io::Result<()> {
    eprintln!("ERROR: Could not serve {}", request.url());

    request.respond(Response::from_string("404")
                    .with_status_code(StatusCode(404)))
}

pub fn serve_static_file(request: Request, filepath: &str) -> io::Result<()> {
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
    request.respond(Response::from_file(file).with_header(content_type_header))
}

pub fn serve_query(request: Request) -> io::Result<()> {
    let content = request.url().split("?q=").last().unwrap().to_string();
    println!("GOT: {} | {:?}", request.url(), content);

    let content_type_header = Header::from_bytes("Content-Type", "text/html; charset=utf8")
        .expect("Garbage on content-type headers!!!!");

    request.respond(Response::from_string(format!(r#"
    <!DOCTYPE html>
    <html>
        <head>
            <title>Googo</title>
        </head>

        <body>
            <section class="logo">
                <div>
                    <a href='/'><img src="assets/logo.png" alt="Googo logo"></a>
                    <h1>Googo</h1>
                </div>
            </section>

            <link rel="stylesheet" href="style.css" type="text/css">
            <div class="result">
                <a href="https://duckduckgo.com/?t=ffab&q=%21g+{content}" target="_blank">Click on this unsuspicious link</a>
            </div>
        </body>
    </html>
    "#)).with_header(content_type_header))
}

pub fn serve_request(request: Request) -> io::Result<()> {
    println!("INFO: received request. method: {:?}, url: {:?}", request.method(), request.url());

    if *request.method() == Method::Get && request.url().starts_with("/query?") {
        return serve_query(request);
    }

    match (request.method(), request.url()) {
        (Method::Get, "/") | (Method::Get, "/index.html") => {
            serve_static_file(request, "index.html")
        },
        (Method::Get, "/style.css") => {
            serve_static_file(request, "style.css")
        },
        (Method::Get, "/no.html") => {
            serve_static_file(request, "no.html")
        },
        (Method::Get, "/assets/spread-love.png") => {
            serve_static_file(request, "assets/spread-love.png")
        },
        (Method::Get, "/assets/native.png") => {
            serve_static_file(request, "assets/native.png")
        },
        (Method::Get, "/assets/nona.png") => {
            serve_static_file(request, "assets/nona.png")
        },
        (Method::Get, "/assets/logo.png") => {
            serve_static_file(request, "assets/logo.png")
        },
        (Method::Get, "/assets/heart.svg") => {
            serve_static_file(request, "assets/heart.svg")
        },
        (Method::Get, "/assets/teaser.png") => {
            serve_static_file(request, "assets/teaser.png")
        },
        (Method::Get, "/assets/arrow.svg") => {
            serve_static_file(request, "assets/arrow.svg")
        },
        _ => {
            serve_404(request)
        }
    }
}

pub fn make_server(addr: &str) -> Result<Server, ()> {
    let server = Server::http(addr).map_err(|err| {
        eprintln!("ERROR: Could not create server: {err}")
    })?;

    Ok(server)
}
