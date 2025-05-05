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

use crate::infrastructure::router::Method;
use fastcgi::Request as FCGI_Request;
use std::collections::HashMap;

pub struct Request {
    method: Method,
    uri: String,
}

impl Request {
    pub fn new(method: Method, uri: &str) -> Self {
        Self {
            method,
            uri: uri.to_string(),
        }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn uri(&self) -> &String {
        &self.uri
    }
}

impl Request {
    pub fn create_from_fcgi_request(request: &FCGI_Request) -> Self {
        let param_map: HashMap<_, _> = request.params().collect();
        let method_str = param_map.get("REQUEST_METHOD");

        Self::new(
            Method::from_str(method_str.unwrap().as_str()).unwrap_or(Method::Get),
            param_map.get("REQUEST_URI").unwrap(),
        )
    }
}
