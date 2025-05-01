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

use super::Controller;
use crate::interfaces::http::response::{HtmlResponse, Response};

pub struct HomeController {}

impl HomeController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for HomeController {
    fn run(&self) -> Box<dyn Response> {
        Box::new(HtmlResponse::new(
            200,
            vec!["Content-Type: text/html".into()],
            "<h1>HomeController::run() v3</h1><br><a href=\"/about.fcgi\">about</> | <a href=\"/guest-book.fcgi\">guest book</>".into(),
        ))
    }
}
