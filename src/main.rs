mod server;

fn main() -> Result<(), ()>{
    let server = server::make_server("0.0.0.0:6969")?;
    let port = server.server_addr().to_ip().unwrap().port();

    println!("Listening at port: {port}");


    for request in server.incoming_requests() {
        server::serve_request(request).map_err(|err| {
            eprintln!("ERROR: Could not serve the response: {err}");
        }).ok();
    }

    Err(())
}
