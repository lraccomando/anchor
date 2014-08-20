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
