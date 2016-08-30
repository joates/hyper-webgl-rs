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

fn get_file(file_name: String) -> Result<String, io::Error> {
    let mut file = try!(File::open(file_name));
    let mut body = String::new();
    try!(file.read_to_string(&mut body));

    Ok(body)
}

// The HTTP server handler
fn handler(req: Request, mut res: Response) {

    let (body, mime) = match &*req.uri.to_string() {

        // permitted routes
        "/" | "/index.html" => {
            println!("\trequest: public/index.html");

            let _body = match get_file(String::from("public/index.html")) {
                Ok(b) => b,
                Err(e) => {
                    format!("Read file error: {}", e)
                }
            };

            let _mime = Mime(TopLevel::Text, SubLevel::Html,
                                 vec![(Attr::Charset, Value::Utf8)]
            );

            (_body, _mime)
        }

        "/js/three.min.js" => {
            println!("\trequest: public/js/three.min.js");

            let _body = match get_file(String::from("public/js/three.min.js")) {
                Ok(b) => b,
                Err(e) => {
                    format!("Read file error: {}", e)
                }
            };

            let _mime = Mime(TopLevel::Application, SubLevel::Javascript,
                                 vec![(Attr::Charset, Value::Utf8)]
            );

            (_body, _mime)
        }

        "/js/OrbitControls.js" => {
            println!("\trequest: public/js/OrbitControls.js");

            let _body = match get_file(String::from("public/js/OrbitControls.js")) {
                Ok(b) => b,
                Err(e) => {
                    format!("Read file error: {}", e)
                }
            };

            let _mime = Mime(TopLevel::Application, SubLevel::Javascript,
                                 vec![(Attr::Charset, Value::Utf8)]
            );

            (_body, _mime)
        }

        _ => { (String::from("oops !"), Mime(TopLevel::Text, SubLevel::Plain,
                                 vec![(Attr::Charset, Value::Utf8)])) }
    };

    res.headers_mut().set(ContentType(mime));
    let mut res = res.start().unwrap();
    res.write_all(body.as_bytes()).unwrap();
    //println!("\t{:?}", &res.headers());
    res.end().unwrap();
}
