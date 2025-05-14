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

use crate::framework::router::Method;

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
