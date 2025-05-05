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

use crate::framework::{
    controller::Controller,
    response::{ Response, HtmlResponse},
    views::View
};

use std::collections::HashMap;

pub struct AboutController {
    view: Box<dyn View>,
}

impl AboutController {
    pub fn new(view: Box<dyn View>) -> Self {
        Self { view }
    }
}

impl Controller for AboutController {
    fn run(&self) -> Box<dyn Response> {
        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("page_title", "Rubala - About");
        params.insert("heading", "About Rubala");
        params.insert("controller_name", "AboutController");
        params.insert("version", "5");

        match self.view.render("layout.html", Some(params)) {
            Ok(content) => Box::new(HtmlResponse::new(
                200,
                vec!["Content-Type: text/html".into()],
                content.as_str(),
            )),
            Err(err) => {
                let error_message = err.to_string();

                Box::new(HtmlResponse::new(
                    500,
                    vec!["Content-Type: text/plain".into()],
                    error_message.as_str(),
                ))
            }
        }
    }
}
