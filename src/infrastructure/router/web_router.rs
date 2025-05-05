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

use super::Router;
use super::WebRoute;
use crate::interfaces::http::request::Request;

pub struct WebRouter {
    routes: HashMap<String, WebRoute>,
}

impl WebRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
}

impl Router for WebRouter {
    fn add_route(&mut self, route: WebRoute) {
        self.routes.insert(route.path.to_string(), route);
    }

    fn match_route(&self, request: &Request) -> Option<&WebRoute> {
        self.routes
            .get(request.uri())
            .filter(|route| route.methods.contains(request.method()))
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::router::method::Method;
    use crate::infrastructure::router::{Request, Router, WebRoute, WebRouter};
    use crate::interfaces::http::controllers::HomeController;
    use crate::interfaces::http::views::HtmlView;

    #[test]
    fn match_route_failure() {
        let request = &Request::new(Method::Get, "/x");
        let mut router = WebRouter::new();

        router.add_route(
            WebRoute::new(
                "home",
                "/",
                vec![Method::Get],
                Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
            )
            .unwrap(),
        );

        let route = router.match_route(request);

        assert!(route.is_none());
    }

    #[test]
    fn match_route_success() {
        let request = &Request::new(Method::Get, "/");
        let mut router = WebRouter::new();

        router.add_route(
            WebRoute::new(
                "home",
                "/",
                vec![Method::Get],
                Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
            )
            .unwrap(),
        );

        let route = router.match_route(request).unwrap();

        assert_eq!("home", route.name);
        assert_eq!("/", route.path);
    }
}
