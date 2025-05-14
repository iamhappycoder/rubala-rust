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
    controller::{Controller, ControllerConstructor},
    router::Method,
};

pub struct WebRoute {
    pub name: String,
    pub pattern: String,
    pub methods: Vec<Method>,
    pub controller: ControllerConstructor,
}

impl WebRoute {
    pub fn new(
        name: &str,
        pattern: &str,
        methods: Vec<Method>,
        controller: Box<dyn Fn() -> Box<dyn Controller>>,
    ) -> Result<Self, &'static str> {
        if methods.is_empty() {
            Err("A route must have at least one method")
        } else {
            Ok(Self {
                name: name.to_string(),
                pattern: pattern.to_string(),
                methods,
                controller,
            })
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::framework::controller::HomeController;
//     use crate::framework::views::HtmlView;
//
//     #[test]
//     fn get_route_success() {
//         let route = WebRoute::new(
//             "home",
//             "/",
//             vec![Method::Get],
//             Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
//         )
//         .unwrap();
//
//         assert_eq!(route.name, "home");
//         assert_eq!(route.path, "/");
//         assert_eq!(route.methods, vec![Method::Get]);
//     }
//
//     #[test]
//     fn post_route_success() {
//         let route = WebRoute::new(
//             "home",
//             "/",
//             vec![Method::Post],
//             Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
//         )
//         .unwrap();
//
//         assert_eq!(route.name, "home");
//         assert_eq!(route.path, "/");
//         assert_eq!(route.methods, vec![Method::Post]);
//     }
//
//     #[test]
//     fn mixed_route_success() {
//         let route = WebRoute::new(
//             "home",
//             "/",
//             vec![Method::Get, Method::Post],
//             Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {})))),
//         )
//         .unwrap();
//
//         assert_eq!(route.name, "home");
//         assert_eq!(route.path, "/");
//         assert_eq!(route.methods, vec![Method::Get, Method::Post]);
//     }
// }
