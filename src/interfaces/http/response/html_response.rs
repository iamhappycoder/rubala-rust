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
use super::Response;
use std::fmt::Display;

pub struct HtmlResponse {
    status_code: u16,
    headers: Vec<String>,
    body: String,
}

impl HtmlResponse {
    pub fn new(status_code: u16, headers: Vec<String>, body: &str) -> Self {
        Self {
            status_code,
            headers,
            body: body.to_string(),
        }
    }
}

impl Response for HtmlResponse {
    fn status_code(&self) -> u16 {
        self.status_code
    }

    fn headers(&self) -> &Vec<String> {
        &self.headers
    }

    fn body(&self) -> &String {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getters_success() {
        let html_response =
            HtmlResponse::new(200, vec!["Content-Type: text/plain".into()], "Hello");

        assert_eq!(html_response.status_code(), 200);
        assert_eq!(html_response.headers(), &vec!["Content-Type: text/plain"]);
        assert_eq!(html_response.body(), "Hello");
    }
}
