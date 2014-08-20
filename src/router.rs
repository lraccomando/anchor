#![feature(phase)]
#[phase(syntax)]
extern crate regex_macros;
extern crate regex;

use controller::Controller;
use regex::Regex;


fn clone<T: Clone>(t: &T) -> T { t.clone() }


pub struct Route {
    path: String,
    matcher: Regex,
    controller: Box<Controller + Send>
}

impl Route {
    pub fn new(path: String, controller: Box<Controller + Send>) -> Route {
        let matcher = Route::compile(path.clone());
        Route { path: path, controller: controller, matcher: matcher }
    }

    pub fn matches(&self, string: &String) -> bool {
        if self.matcher.is_match(string.as_slice()) {
            true
        } else {
            false
        }
    }

    fn compile(path: String) -> Regex {
        let mut rule = String::new();
        rule.push_str("/?");

        for part in path.as_slice().split('/') {
            if part.is_empty() {
                continue
            }
            if part.starts_with(":") {
                let name = format!("(?P<{}>[a-zA-Z_][a-zA-Z0-9_]*)", part.slice_from(1));
                rule.push_str(name.as_slice());
            } else {
                rule.push_str(part);
            }
            rule.push_str("/");
        }
        rule.push_str("?");

        println!("{}", rule);
        // @TODO -- Have real error handling here
        match Regex::new(rule.as_slice()) {
            Ok(re) => re,
            Err(err) => regex!("")
        }
    }
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route {
            path: clone(&self.path),
            controller: clone(&self.controller),
            matcher: clone(&self.matcher)
        }
    }
}


pub trait Router: Send + Clone {

    fn register<C: Controller>(&mut self, path: String, controller: C);

    fn new() -> Self;

    fn match_path(&mut self, path: &String) -> Option<&Box<Controller + Send>>;
}


#[deriving(Clone)]
pub struct DefaultRouter {
    routes: Vec<Route>
}

impl Router for DefaultRouter {
    fn new() -> DefaultRouter {
        DefaultRouter { routes: Vec::new() }
    }

    fn register<C: Controller>(&mut self, path: String, controller: C) {
        let route = Route::new(path, box controller);
        self.routes.push(route);
    }

    fn match_path(&mut self, path: &String) -> Option<&Box<Controller + Send>> {
        for route in self.routes.iter() {
            if route.matches(path) {
                return Some(&route.controller)
            }
        }
        None
    }
}
