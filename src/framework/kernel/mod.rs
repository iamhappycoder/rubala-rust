/*
 * Rubala
 *
 * Copyright (c) 2025, Jasper V. Ferrer
 *
 * This file is part of the Rubala source code.
 *
 * For licensing information, please see the LICENSE file distributed with this code.
 * For inquiries or support, please visit the project's repository at https://github.com/iamhappycoder/rubala.
 */

use std::collections::HashMap;
use std::io::Write;
use fastcgi::Request as FcgiRequest;

use crate::framework::{
    router::{Router, WebRouter, WebRoute},
};
use crate::framework::request::Request;
use crate::framework::response::Response;
use crate::framework::router::Method;

pub struct Kernel {
    router: WebRouter,
}

impl Kernel {
    pub fn boot(routes: Vec<Result<WebRoute, &str>>, mut fcgi_request: FcgiRequest) {
        let mut web_router = WebRouter::new();
        for route in routes.into_iter() {
            web_router.add_route(route.unwrap());
        }

        let fcgi_params_map: HashMap<String, String> = fcgi_request.params().collect();
        let method_str = fcgi_params_map.get("REQUEST_METHOD");
        let request_uri = fcgi_params_map.get("REQUEST_URI");

        let request = &Request::new(
            Method::from_str(method_str.unwrap().as_str()).unwrap_or(Method::Get),
            request_uri.unwrap(),
        );

        let response = Kernel::new(web_router).run(request);

        let mut headers: String = String::new();
        headers.push_str(&format!("Status: {} OK", response.status_code()));
        headers.push_str(&response.headers().join("\r\n"));

        let mut all_params = String::new();
        for (key, value) in fcgi_params_map {
            all_params.push_str(&format!("{}: {}\n", key.trim(), value.trim()));
        }

        if let Err(_e) = write!(
            fcgi_request.stdout(),
            "{}\r\n\r\n{}\n<pre>{}</pre>",
            headers,
            response.body(),
            all_params
        ) {
            // Error handling
        }
    }
    
    pub fn new(
        router: WebRouter,
    ) -> Self {
        Self {
            router,
        }
    }
    
    pub fn run(&mut self, request: &Request) -> Box<dyn Response> {
        let route = match self.router.match_route(request) {
            Some(route) => route,
            None => panic!("Route not match"),
        };

        (route.controller)().run()
    }
}