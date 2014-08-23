extern crate anchor;
extern crate http;
extern crate iron;
extern crate typemap;
extern crate url;

use anchor::{Controller, Response, Request, Body};
use http::{method, status};
use http::headers::{request, response};
use typemap::TypeMap;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use iron::Url as IronUrl;
use iron::Response as HttpResponse;
use url::Url;


//@TODO -- Move these into a 'test_utils' module of some sort.
fn mock_request(method: method::Method) -> Result<Request, String> {
    let url = match url::Url::parse("http://127.0.0.1:3000") {
        Ok(url) => url,
        Err(e) => return Err("Error Parsing 1".to_string()),
    };

    let url = match IronUrl::from_generic_url(url) {
        Ok(url) => url,
        Err(_) => return Err("Error Parsing 2".to_string())
    };

    Ok(Request {url: url,
                remote_addr: Some(SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 3000 }),
                headers: box request::HeaderCollection::new(),
                body: "Testing".to_string(),
                method: method,
                extensions: TypeMap::new()
               }
    )
}

fn mock_response() -> HttpResponse {
    HttpResponse {
        headers: box response::HeaderCollection::new(),
        status: None,
        body: None,
        extensions: TypeMap::new(),
    }
}

#[test]
fn test_mock_request_works_okay() {
    match mock_request(method::Get) {
        Ok(request) => {
            assert_eq!(request.method, method::Get);
            assert_eq!(request.body, "Testing".to_string());
            assert_eq!(request.remote_addr, Some(SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 3000 }));
        },
        Err(e) => fail!(e),
    };
}

#[deriving(Clone)]
struct TestGetController;

impl Controller for TestGetController {
    fn get(&self, request: &mut Request) -> Response {
        Body("Get Was Successfully Hit".to_string())
    }
}

#[test]
fn test_controller_dispatches_get_properly() {
    let mut request = match mock_request(method::Get) {
        Ok(request) => request,
        Err(e) => fail!(e)
    };

    let mut response = mock_response();

    let controller = TestGetController;
    controller.dispatch(&mut request, &mut response);

    match response.body.unwrap().read_to_string() {
        Ok(body) => { assert_eq!(body, "Get Was Successfully Hit".to_string()); },
        Err(e) => fail!("Failed to Read Body")
    }

    assert_eq!(response.status.unwrap(), status::Ok);
}
