use http::{status, method};
use iron::{Request, Status, Unwind, Url};
use iron::Response as HttpResponse;


pub enum Response {
    Body(&'static str),
    Status(&'static str, status::Status),
}


pub trait Controller: Send + Clone {
    fn get(&self, request: &mut Request) -> Response;

    fn dispatch(&self, request: &mut Request, response: &mut HttpResponse) -> Status {
        let output = match request.method {
            method::Get => { self.get(request) },
            _ => { Status("Please Try Again.", status::MethodNotAllowed)}
        };

        match output {
            Body(body) => { response.serve(status::Ok, body); },
            Status(body, status) => { response.serve(status, body); }
        };

        Unwind
    }

    fn clone_box(&self) -> Box<Controller + Send> {
        box self.clone() as Box<Controller + Send>
    }
}


impl Clone for Box<Controller + Send> {
    fn clone(&self) -> Box<Controller + Send> { self.clone_box() }
}


#[cfg(test)]
mod test {
    use super::{Controller, Response, Body};
    use http::{method, status};
    use http::headers::{request, response};
    use anymap::AnyMap;
    use std::io::net::ip::{Ipv4Addr, SocketAddr};
    use iron::{Request, Url};
    use iron::Response as HttpResponse;
    use url;


    //@TODO -- Move these into a 'test_utils' module of some sort.
    fn mock_request(method: method::Method) -> Result<Request, String> {
        let url = match url::Url::parse("http://127.0.0.1:3000") {
            Ok(url) => url,
            Err(e) => return Err("Error Parsing 1".to_string()),
        };

        let url = match Url::from_generic_url(url) {
            Ok(url) => url,
            Err(_) => return Err("Error Parsing 2".to_string())
        };

        Ok(Request {url: url,
                    remote_addr: Some(SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 3000 }),
                    headers: box request::HeaderCollection::new(),
                    body: "Testing".to_string(),
                    method: method,
                    extensions: AnyMap::new()
                   }
        )
    }

    fn mock_response() -> HttpResponse {
        HttpResponse {
            headers: box response::HeaderCollection::new(),
            status: None,
            body: None,
            extensions: AnyMap::new(),
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
            Body("Get Was Successfully Hit")
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
}
