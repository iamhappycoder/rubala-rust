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
use regex::Regex;
use once_cell::sync::Lazy;

use crate::framework::{
    request::Request,
    router::{Router, WebRoute},
};

static PLACEHOLDER_RE: Lazy<Regex> = Lazy::new(|| {
    // Regex to capture:
    // 1. The full placeholder e.g. {id} or {id:int} or {name:[a-z]+}
    // 2. The name inside the placeholder e.g. id or name
    // 3. Optionally, a type/constraint part after a colon e.g. :int or :[a-z]+
    Regex::new(r"^\{([a-zA-Z_][a-zA-Z0-9_]*)(?::([^}]+))?\}$").unwrap()
});

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
        self.routes.insert(route.pattern.to_string(), route);
    }

    fn match_route(&self, request: &Request) -> Option<&WebRoute> {
        self.routes
            .get(request.uri())
            .filter(|route| route.methods.contains(request.method()))
    }

    fn match_path_segments(pattern: &str, uri: &str) -> Option<HashMap<String, String>> {
        let pattern_segments: Vec<&str> = pattern.trim_matches('/').split('/').collect();
        let uri_segments: Vec<&str> = uri.trim_matches('/').split('/').collect();

        if pattern_segments.len() != uri_segments.len() {
            return None;
        }

        let mut extracted_params = HashMap::new();

        for (p_segment, u_segment) in pattern_segments.iter().zip(uri_segments.iter()) {
            if let Some(captures) = PLACEHOLDER_RE.captures(p_segment) {
                let param_name = captures.get(1).map_or("", |m| m.as_str());
                extracted_params.insert(param_name.to_string(), u_segment.to_string());
            } else if p_segment != u_segment {
                return None;
            }

        }
        Some(extracted_params)
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::infrastructure::router::method::Method;
//     use crate::infrastructure::router::{Request, Router, WebRoute, WebRouter};
//     use crate::interfaces::http::controllers::HomeController;
//     use crate::interfaces::http::views::HtmlView;
// 
//     #[test]
//     fn match_route_failure() {
//         let request = &Request::new(Method::Get, "/x");
//         let mut router = WebRouter::new();
// 
//         router.add_route(
//             WebRoute::new(
//                 "home",
//                 "/",
//                 vec![Method::Get],
//                 Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
//             )
//             .unwrap(),
//         );
// 
//         let route = router.match_route(request);
// 
//         assert!(route.is_none());
//     }
// 
//     #[test]
//     fn match_route_success() {
//         let request = &Request::new(Method::Get, "/");
//         let mut router = WebRouter::new();
// 
//         router.add_route(
//             WebRoute::new(
//                 "home",
//                 "/",
//                 vec![Method::Get],
//                 Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
//             )
//             .unwrap(),
//         );
// 
//         let route = router.match_route(request).unwrap();
// 
//         assert_eq!("home", route.name);
//         assert_eq!("/", route.path);
//     }
// }
