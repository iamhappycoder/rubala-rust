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

mod infrastructure;
mod interfaces;
mod framework;

use String;
use fastcgi;
use std::collections::HashMap;
use std::io::Write;

use crate::framework::{
    request::Request,
    views::HtmlView,
    router::{Method, Router, WebRoute, WebRouter},
};

use interfaces::http::controllers::{AboutController, GuestBookController, HomeController};

fn main() {
    fastcgi::run_once(|mut fcgi_request| {
        let routes = vec![
            WebRoute::new(
                "home",
                "/",
                vec![Method::Get],
                Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
            ),
            WebRoute::new(
                "home",
                "/users/1",
                vec![Method::Post],
                Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
            ),
            WebRoute::new(
                "about",
                "/about",
                vec![Method::Get],
                Box::new(|| Box::new(AboutController::new(Box::new(HtmlView {})))),
            ),
            WebRoute::new(
                "guest_book",
                "/guest-book",
                vec![Method::Get],
                Box::new(|| Box::new(GuestBookController::new(Box::new(HtmlView {})))),
            ),
        ];

        let mut web_router = WebRouter::new();

        for route in routes.into_iter() {
            web_router.add_route(route.unwrap());
        }

        let param_map: HashMap<_, _> = fcgi_request.params().collect();
        let mut all_params = String::new();
        for (key, value) in param_map {
            all_params.push_str(&format!("{}: {}\n", key.trim(), value.trim()));
        }

        let request = &Request::create_from_fcgi_request(&fcgi_request);
        let route = match web_router.match_route(request) {
            Some(route) => route,
            None => return,
        };

        let r = (route.controller)().run();

        let mut stdout = fcgi_request.stdout();
        let mut headers: String = String::new();

        headers.push_str(&format!("Status: {} OK", r.status_code()));
        headers.push_str(&r.headers().join("\r\n"));

        if let Err(_e) = write!(
            stdout,
            "{}\r\n\r\n{}\n<pre>{}</pre>",
            headers,
            r.body(),
            all_params
        ) {
            // Error handling
        }
    });
}
