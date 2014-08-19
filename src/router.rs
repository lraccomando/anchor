use controller::Controller;


fn clone<T: Clone>(t: &T) -> T { t.clone() }


pub struct Route {
    path: String,
    controller: Box<Controller + Send>
}

impl Route {
    pub fn matches(&self, string: &String) -> bool {
        if *string == self.path {
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
        let route = Route { path: path, controller: box controller };
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
