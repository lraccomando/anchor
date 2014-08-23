extern crate iron;
extern crate anchor;

use iron::{Iron, Chain, Server};
use anchor::{Anchor, App, Body, Controller, Response, Request};


fn main () {
    #[deriving(Clone)]
    struct HelloWorld;
    impl Controller for HelloWorld {
        fn get(&self, request: &mut Request) -> Response {
            Body("Hello, World".to_string())
        }
    }

    #[deriving(Clone)]
    struct HelloName;
    impl Controller for HelloName {
        fn get(&self, request: &mut Request) -> Response {
            let greeting = "Hello, ".to_string() + request.get_param("name");
            Body(greeting)
        }
    }

    let mut app: App = Anchor::new();
    app.register("hello/:name/", HelloName);
    app.register("hello/", HelloWorld);
    app.run()
}
