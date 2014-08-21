#![feature(phase)]
#[phase(syntax)]
extern crate regex_macros;
extern crate regex;

use std::collections::hashmap::HashMap;

use controller::Controller;
use regex::Regex;


fn clone<T: Clone>(t: &T) -> T { t.clone() }


pub struct Route {
    path: String,
    matcher: Regex,
    controller: Box<Controller + Send>,
    named_groups: Vec<String>,
}

impl Route {
    pub fn new(path: String, controller: Box<Controller + Send>) -> Route {
        let (matcher, named_groups) = Route::compile(path.clone());
        Route { path: path, controller: controller, matcher: matcher, named_groups: named_groups }
    }

    pub fn matches(&self, string: &String) -> bool {
        if self.matcher.is_match(string.as_slice()) {
            true
        } else {
            false
        }
    }

    pub fn params(&mut self, string: &String) -> HashMap<String, String> {
        let mut params = HashMap::new();
        for capture in self.matcher.captures_iter(string.as_slice()) {
            for group in self.named_groups.move_iter() {
                params.insert(group.clone(), capture.name(group.as_slice()).to_string());
            }
        }
        params
    }

    fn compile(path: String) -> (Regex, Vec<String>) {
        let mut rule = String::new();
        let mut groups = Vec::new();
        rule.push_str("/?");

        for part in path.as_slice().split('/') {
            if part.is_empty() {
                continue
            }
            if part.starts_with(":") {
                let group = part.slice_from(1);
                groups.push(group.to_string());
                let name = format!("(?P<{}>[a-zA-Z_][a-zA-Z0-9_]*)", group);

                rule.push_str(name.as_slice());
            } else {
                rule.push_str(part);
            }
            rule.push_str("/");
        }
        rule.push_str("?");

        // @TODO -- Have real error handling here
        match Regex::new(rule.as_slice()) {
            Ok(re) => (re, groups),
            Err(err) => (regex!(""), Vec::new())
        }
    }
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route {
            path: clone(&self.path),
            controller: clone(&self.controller),
            matcher: clone(&self.matcher),
            named_groups: clone(&self.named_groups),
        }
    }
}


pub trait Router: Send + Clone {

    fn register<C: Controller>(&mut self, path: String, controller: C);

    fn new() -> Self;

    fn match_path(&mut self, path: &String) -> Option<(&Box<Controller + Send>, HashMap<String, String>)>;
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

    fn match_path(&mut self, path: &String) -> Option<(&Box<Controller + Send>, HashMap<String, String>)> {
        for route in self.routes.iter() {
            if route.matches(path) {
                return Some((&route.controller, route.params(path)))
            }
        }
        None
    }
}
