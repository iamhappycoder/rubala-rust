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
use std::collections::HashMap;

pub struct GuestBookController {
    view: Box<dyn View>,
}

impl GuestBookController {
    pub fn new(view: Box<dyn View>) -> Self {
        Self { view }
    }
}

impl Controller for GuestBookController {
    fn run(&self) -> Box<dyn Response> {
        let mut params = HashMap::new();
        params.insert("page_title", "Rubala - Guest Book");
        params.insert("heading", "Sign the guest book");
        params.insert("controller_name", "GuestBookController");
        params.insert("version", "5");

        match self.view.render("layout.html", Some(params)) {
            Ok(content) => Box::new(HtmlResponse::new(
                200,
                vec!["Content-Type: text/html".to_string()],
                content.as_str(),
            )),
            Err(err) => {
                let error_message = err.to_string();

                Box::new(HtmlResponse::new(
                    404,
                    vec!["Content-Type: text/plain".into()],
                    error_message.as_str(),
                ))
            }
        }
    }
}
