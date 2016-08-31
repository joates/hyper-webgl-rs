use std::thread;
use std::io::{self, Write, Read};
use std::fs::File;

use hyper::Server as HttpServer;
use hyper::server::request::Request;
use hyper::server::response::Response;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

pub fn start() {
    // Start listening for http connections
    thread::spawn(move || {
        let http_server = HttpServer::http("127.0.0.1:8080").unwrap();
        http_server.handle(handler).unwrap();
    });
}

// The HTTP server handler
fn handler(req: Request, mut res: Response) {
    let (body, mime) = match &*req.uri.to_string() {

        // permitted routes
        "/js/three.min.js" => load_js("three.min.js"),
        "/js/OrbitControls.js" => load_js("OrbitControls.js"),

        // all other route requests return `index.html`
        _ => load_index()
    };
    res.headers_mut().set(ContentType(mime));
    let mut res = res.start().unwrap();
    res.write_all(body.as_bytes()).unwrap();
    res.end().unwrap();
}

fn load_index() -> (String, Mime) {
    println!("\trequest: public/index.html");

    let body = match get_file(String::from("public/index.html")) {
        Ok(b) => b,
        Err(e) => {
            format!("Read file error: {}", e)
        }
    };
    let mime_html = Mime(TopLevel::Text, SubLevel::Html,
                         vec![(Attr::Charset, Value::Utf8)]
    );
    (body, mime_html)
}

fn load_js(file: &str) -> (String, Mime) {
    println!("\trequest: public/js/{}", file);

    let body = match get_file("public/js/".to_string() + file) {
        Ok(b) => b,
        Err(e) => {
            format!("Read file error: {}", e)
        }
    };
    let mime_js = Mime(TopLevel::Application, SubLevel::Javascript,
                         vec![(Attr::Charset, Value::Utf8)]
    );
    (body, mime_js)
}

fn get_file(file_name: String) -> Result<String, io::Error> {
    let mut file = try!(File::open(file_name));
    let mut body = String::new();
    try!(file.read_to_string(&mut body));
    Ok(body)
}
