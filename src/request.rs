use std::io::net::ip::SocketAddr;
use std::collections::hashmap::HashMap;

use http::headers::request::HeaderCollection;
use http::method::Method;
use iron::Request as IronRequest;
use iron::Url;

use anymap::AnyMap;


pub struct Request {
	pub url: Url,
	pub remote_addr: Option<SocketAddr>,
	pub headers: Box<HeaderCollection>,
	pub body: String,
	pub method: Method,
	pub params: HashMap<String, String>,
	pub extensions: AnyMap,
}


impl Request {
	pub fn from_iron(request: &mut IronRequest) -> Request {
		Request {
			url: request.url,
			remote_addr: request.remote_addr,
			headers: request.headers,
			body: request.body,
			method: request.method,
			params: HashMap::new(),
			extensions: request.extensions,
		}
	}

	pub fn set_params(&mut self, params: HashMap<String, String>) {
		self.params = params;
	}
}
