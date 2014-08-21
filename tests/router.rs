extern crate anchor;
extern crate iron;

use anchor::{Body, Controller, Response, Route, Router};
use iron::Request;

// Tests for the Route Struct

#[deriving(Clone)]
struct TestController;
impl Controller for TestController {
    fn get(&self, request: &mut Request) -> Response {
        Body("This worked!")
    }
}

#[test]
fn test_very_basic_route_matching_works() {
    let path = "/test".to_string();
    let mut route = Route::new(path.clone(), box TestController);
    assert!(route.matches(&path));
}

#[test]
fn test_routes_with_multiple_trailing_slashes_work() {
    let path = "/testing/slashes/sup/".to_string();
    let mut route = Route::new(path.clone(), box TestController);
    assert!(route.matches(&path));
}

#[test]
fn test_routes_with_named_params_work() {
    let mut route = Route::new("/testing/:param/".to_string(), box TestController);
    assert!(route.matches(&"/testing/thing/".to_string()));
}

#[test]
fn test_routes_can_have_optional_leading_slashes() {
    let mut route = Route::new("/test".to_string(), box TestController);
    assert!(route.matches(&"/test".to_string()));
    assert!(route.matches(&"test".to_string()));
}

#[test]
fn test_routes_can_have_optional_trailing_slashes() {
    let mut route = Route::new("/test".to_string(), box TestController);
    assert!(route.matches(&"test/".to_string()));
    assert!(route.matches(&"test".to_string()));
}

#[test]
fn test_empty_routes_work_as_expected() {
    let mut route = Route::new("".to_string(), box TestController);
    assert!(route.matches(&"".to_string()));
    assert!(route.matches(&"/".to_string()));
}

#[test]
fn test_root_routes_work_as_expected() {
    let mut route = Route::new("/".to_string(), box TestController);
    assert!(route.matches(&"".to_string()));
    assert!(route.matches(&"/".to_string()));
}
