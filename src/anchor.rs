extern crate http;
extern crate iron;

use iron::{Middleware, Request, Response, Status, Unwind};

use controller::{Controller, Responses, Body};
use router::{Router, DefaultRouter};

pub type App = Anchor<DefaultRouter>;

#[deriving(Clone)]
pub struct Anchor<R> {
    router: R
}

impl<R: Router> Anchor<R> {
    pub fn new() -> Anchor<R> {
        Anchor { router: Router::new() }
    }
}

#[deriving(Clone)]
struct HelloWorld {
    value: &'static str
}

impl HelloWorld {
    fn new() -> HelloWorld {
        HelloWorld {value: "Hello there, World!"}
    }
}

impl Controller for HelloWorld {
    fn get(&self, request: &mut Request) -> Responses {
        Body("Hello, World")
    }
}

impl<R: Router> Middleware for Anchor<R> {
    fn enter(&mut self, request: &mut Request, response: &mut Response) -> Status {
        self.router.register("hello", HelloWorld::new());
        match self.router.match_path(&request.url.path) {
            Some(controller) => {
                controller.dispatch(request, response)
            }
            None => {
                response.serve(::http::status::Ok, "No Route Found");
                Unwind
            }
        }

    }
}
