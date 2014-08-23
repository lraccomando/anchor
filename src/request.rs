use std::collections::hashmap::HashMap;
use std::io::net::ip::SocketAddr;
use std::any::Any;

use http::headers::request::HeaderCollection;
use http::method::Method;

use typemap::{Assoc, TypeMap};
use iron::Url;
use iron::Request as IronRequest;

fn clone<T: Clone>(t: &T) -> T { t.clone() }

pub struct Params;
impl<'a> Assoc<HashMap<String, String>> for Params {}

pub struct Request {
    pub url: Url,
    pub remote_addr: Option<SocketAddr>,
    pub headers: Box<HeaderCollection>,
    pub body: String,
    pub method: Method,
    pub extensions: TypeMap,
}

impl Request {
    pub fn from_iron(request: &mut IronRequest) -> Request {
        Request {
            url: clone(&request.url),
            remote_addr: clone(&request.remote_addr),
            headers: clone(&request.headers),
            body: clone(&request.body),
            method: clone(&request.method),
            // @TODO -- This is will ignore anything in the extensions. We do not
            // want this. Would prefer that TypeMap implements Clone.
            extensions: TypeMap::new(),
        }
    }

    pub fn set_params(&mut self, params: HashMap<String, String>) {
        self.extensions.insert::<Params, HashMap<String, String>>(params);
    }

    pub fn get_param(&self, param: &str) -> String {
        let params = self.extensions.find::<Params, HashMap<String, String>>().unwrap();
        // @TODO - This needs error checking
        params.find(&param.to_string()).unwrap().clone()
    }
}
