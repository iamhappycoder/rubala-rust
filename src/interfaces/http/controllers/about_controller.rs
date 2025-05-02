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
use super::Controller;
use crate::interfaces::http::response::{HtmlResponse, Response};
use crate::interfaces::http::views::View;

pub struct AboutController {
    view: Box<dyn View>,
}

impl AboutController {
    pub fn new(view: Box<dyn View>) -> Self {
        Self {
            view,
        }
    }
}

impl Controller for AboutController {
    fn run(&self) -> Box<dyn Response> {
        let mut params = HashMap::new();
        params.insert("page_title".into(), "Rubala - About".into());
        params.insert("heading".into(), "About Rubala".into());
        params.insert("controller_name".into(), "AboutController".into());
        params.insert("version".into(), "5".into());

        match self.view.render(&"layout.html".to_string(), Some(params)) {
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
