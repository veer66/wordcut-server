extern crate hyper;
extern crate futures;
extern crate tokio_proto;
#[macro_use]
extern crate serde_json;
extern crate wordcut_engine;

use futures::future;
use futures::Stream;
use futures::Future;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

use tokio_proto::TcpServer;

use serde_json::Value;

use wordcut_engine::{load_dict, Wordcut};

#[macro_use]
extern crate lazy_static;

use std::error;
use std::fmt;
use std::result;
use std::path::Path;


#[derive(Debug)]
pub enum ServerError {
    CannotReadBody,
    CannotParseJsonRequest,
    CannotGetJsonObject,
    CannotGetTextAttr,
    TextAttrIsNotString
}

lazy_static! {
    static ref WORDCUT: Wordcut = Wordcut::new(load_dict(Path::new("wordlist.txt"))
                                               .expect("Cannot load dict"));
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Error: {:?}", self)
    }
}

impl error::Error for ServerError {
    fn description(&self) -> &str {
        "Server error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

struct WordcutServer;

const NOT_FOUND_MSG: &'static str = "Not found";

type WebFuture = Box<future::Future<Item=Response, Error=hyper::Error>>;
type BodyFuture = Box<future::Future<Item=Vec<u8>, Error=Box<ServerError>>>;

fn resp_with_msg(msg: &str) -> Response {
    Response::new()
        .with_header(ContentLength(msg.len() as u64))
        .with_body(String::from(msg))
}

fn read_val(body: Vec<u8>) -> Result<Value, Box<ServerError>> {
    match serde_json::from_slice::<Value>(&body) {
        Ok(val) => Ok(val),
        Err(_) => Err(Box::new(ServerError::CannotParseJsonRequest))
    }
}

fn get_text(val: Value) -> Result<String, Box<ServerError>> {
    val.as_object().ok_or(Box::new(ServerError::CannotGetJsonObject))
        .and_then(|obj| obj.get("text").ok_or(Box::new(ServerError::CannotGetTextAttr)))
        .and_then(|text| text.as_str().ok_or(Box::new(ServerError::CannotGetTextAttr)))
        .map(|text| String::from(text))
}

fn wordseg(text: String) -> Result<Value, Box<ServerError>> {
    let toks = WORDCUT.segment_into_strings(&text);
    Ok(json!({"words": toks}))
}

fn read_body(req: Request) -> BodyFuture {
    let fut = req.body()
        .map_err(|_| Box::new(ServerError::CannotReadBody))
        .fold(vec![], |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Box<ServerError>>(body)
        });
    Box::new(fut)
}

fn make_resp(val: Result<Value, Box<ServerError>>) -> Result<Response, hyper::Error> {
    match val {
        Ok(val) => {
            let s = serde_json::to_string(&val);
            match s {
                Ok(s) => Ok::<_,hyper::Error>(resp_with_msg(&s)),
                Err(e) => Ok::<_,hyper::Error>(resp_with_msg(&format!("Err {}", e)))
            }
        },
        Err(e) => Ok::<_,hyper::Error>(resp_with_msg(&format!("Err {}", e)))
    }
}

fn wordseg_handler(req: Request) -> WebFuture {
    let fut = read_body(req)
        .and_then(read_val)
        .and_then(get_text)
        .and_then(wordseg)
        .then(make_resp);
    return Box::new(fut)
}

fn not_found(_req: Request) -> WebFuture {
    let resp = Response::new()
        .with_header(ContentLength(NOT_FOUND_MSG.len() as u64))
        .with_status(hyper::StatusCode::NotFound)
        .with_body(NOT_FOUND_MSG);
    let fut = futures::future::ok(resp);
    Box::new(fut)                   
}

impl Service for WordcutServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = WebFuture;

    
    fn call(&self, req: Request) -> Self::Future {
        if req.method() == &hyper::Method::Post && req.path() == "/wordseg" {
            wordseg_handler(req)
        } else {
            not_found(req)
        }
    }
}

fn main() {
    let num_threads = 8;
    let addr = "127.0.0.1:3000".parse().unwrap();
    let http_server = Http::new();    
    let mut tcp_server = TcpServer::new(http_server, addr);
    tcp_server.threads(num_threads);

    println!("Listening {:?} ...", addr);
    
    tcp_server.serve(||Ok(WordcutServer));
}
