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
use crate::interfaces::http::views::View;

pub struct HomeController {
    view: Box<dyn View>,
}

impl HomeController {
    pub fn new(view: Box<dyn View>) -> Self {
        Self {
            view,
        }
    }
}

impl Controller for HomeController {
    fn run(&self) -> Box<dyn Response> {
        match self.view.render(&"home.html".to_string(), None) {
            Ok(content) => Box::new(HtmlResponse::new(
                200,
                vec!["Content-Type: text/html".into()],
                content,
            )),
            Err(err) => Box::new(HtmlResponse::new(
                500,
                vec!["Content-Type: text/plain".into()],
                err.to_string(),
            ))
        }
    }
}
