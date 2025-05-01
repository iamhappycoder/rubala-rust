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

use fastcgi;
use std::io::Write;
use String;

use infrastructure::router::{Method, Router, WebRoute, WebRouter};
use interfaces::http::controllers::{AboutController, HomeController, GuestBookController};

fn main() {
    fastcgi::run_once(|mut request| {
        let routes = vec![
            WebRoute::new("home", "/", vec![Method::Get], Box::new(|| Box::new(HomeController::new()))),
            WebRoute::new("about", "/about.fcgi", vec![Method::Get],Box::new(|| Box::new(AboutController::new()))),
            WebRoute::new("guest_book", "/guest-book.fcgi", vec![Method::Get],Box::new(|| Box::new(GuestBookController::new()))),    
        ];

        let mut web_router = WebRouter::new();
        
        for route in routes.into_iter() {
            web_router.add_route(route.unwrap());
        }

        let param = match request.param("REQUEST_URI") {
            Some(param) => param,
            None => return,
        };

        let route = match web_router.match_route(param) {
            Some(route) => route,
            None => return,
        };

        let r = (route.controller)().run();
        
        let mut stdout = request.stdout();
        let mut headers: String = String::new();

        headers.push_str(&format!("Status: {} OK", r.status_code()));
        headers.push_str(&r.headers().join("\r\n"));
        
        if let Err(_e) = write!(stdout, "{}\r\n\r\n{}", headers, r.body()) {
            // Error handling
        }
    });
}
