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

use fastcgi;

use crate::framework::kernel::Kernel;
use crate::framework::router::{Method, WebRoute};
use crate::framework::views::HtmlView;
use crate::interfaces::http::controllers::{AboutController, GuestBookController, HomeController};

fn main() {
    fastcgi::run_once(|fcgi_request| {
        let routes= vec![
            WebRoute::new(
                "home",
                "/",
                vec![Method::Get],
                Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
            ),
            WebRoute::new(
                "home",
                "/users/{id}",
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

        Kernel::boot(routes, fcgi_request);
    });
}
