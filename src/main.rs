extern crate iron;
extern crate anchor;

use iron::{Iron, Chain, Request, Server};

use anchor::{Anchor, App, Body, Controller, Response};

fn main () {
    #[deriving(Clone)]
    struct HelloWorld;

    impl Controller for HelloWorld {
        fn get(&self, request: &mut Request) -> Response {
            Body("Hello, World")
        }
    }

    let mut app: App = Anchor::new();
    app.register("", HelloWorld);
    app.run()
}
