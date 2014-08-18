
use controller::Controller;

pub trait Router: Send + Clone {

    fn register<C: Controller>(&mut self, path: &'static str, controller: C);

    fn new() -> Self;

    fn match_path(&mut self, path: &Vec<String>) -> Option<&Box<Controller + Send>>;
}

fn clone<T: Clone>(t: &T) -> T { t.clone() }

pub struct Route {
    path: &'static str,
    controller: Box<Controller + Send>
}

impl Route {
    pub fn matches(&self, string: String) -> bool {
        if string == self.path.to_string() {
            true
        } else {
            false
        }
    }
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route {
            path: clone(&self.path),
            controller: clone(&self.controller)
        }
    }
}

#[deriving(Clone)]
pub struct DefaultRouter {
    routes: Vec<Route>
}

impl Router for DefaultRouter {
    fn new() -> DefaultRouter {
        DefaultRouter { routes: Vec::new() }
    }

    fn register<C: Controller>(&mut self, path: &'static str, controller: C) {
        let route = Route { path: path, controller: box controller };
        self.routes.push(route);
    }

    fn match_path(&mut self, path: &Vec<String>) -> Option<&Box<Controller + Send>> {
        let path = path.connect("/");
        for route in self.routes.iter() {
            if route.matches(path.clone()) {
                return Some(&route.controller)
            }
        }
        None
    }

}
