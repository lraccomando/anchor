use std::io::net::ip::Ipv4Addr;

use iron::{Chain, Iron, Middleware, Request, Status, Server, Unwind};
use iron::Response as HttpResponse;

use controller::{Controller, Response, Body};
use router::{Route, Router, DefaultRouter};


pub type App = Anchor<DefaultRouter>;


pub struct Anchor<R> {
    server: Server,
    anchor: AnchorMiddleware<R>,
}

impl<R: Router> Anchor<R> {
    pub fn new() -> Anchor<R> {
        Anchor { server: Iron::new(), anchor: AnchorMiddleware::new() }
    }

    pub fn register<C: Controller>(&mut self, path: &'static str, controller: C) {
        self.anchor.register(path.to_string(), controller);
    }

    pub fn run(mut self) {
        self.server.chain.link(self.anchor);
        self.server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
    }
}


#[deriving(Clone)]
struct AnchorMiddleware<R> {
    router: R
}

impl<R: Router> AnchorMiddleware<R> {
    pub fn new() -> AnchorMiddleware<R> {
        AnchorMiddleware { router: Router::new() }
    }

    pub fn register<C: Controller>(&mut self, path: String, controller:C) {
        self.router.register(path, controller);
    }
}

impl<R: Router> Middleware for AnchorMiddleware<R> {
    fn enter(&mut self, request: &mut Request, response: &mut HttpResponse) -> Status {
        let path = request.url.path.connect("/");
        match self.router.match_path(&path) {
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
