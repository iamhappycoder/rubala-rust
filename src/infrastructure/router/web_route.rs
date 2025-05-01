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

use crate::interfaces::http::controllers::{ControllerConstructor, Controller};
use super::Method;

pub struct WebRoute {
    pub name: String,
    pub path: String,
    pub methods: Vec<Method>,
    pub controller: ControllerConstructor,
}

impl WebRoute {
    pub fn new<T: Into<String>>(name: T, path: T, methods: Vec<Method>, controller: Box<dyn Fn() -> Box<dyn Controller>>) -> Result<Self, String> {
        if methods.is_empty() {
            Err("A route must have at least one method".to_string())
        } else {
            Ok(Self {
                name: name.into(),
                path: path.into(),
                methods,
                controller,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interfaces::http::controllers::HomeController;
    use crate::interfaces::http::views::HtmlView;

    #[test]
    fn get_route_success() {
        let route = WebRoute::new("home", "/", vec![Method::Get], Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {}))))).unwrap();

        assert_eq!(route.name, "home");
        assert_eq!(route.path, "/");
        assert_eq!(route.methods, vec![Method::Get]);
    }

    #[test]
    fn post_route_success() {
        let route = WebRoute::new("home", "/", vec![Method::Post], Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {}))))).unwrap();

        assert_eq!(route.name, "home");
        assert_eq!(route.path, "/");
        assert_eq!(route.methods, vec![Method::Post]);
    }

    #[test]
    fn mixed_route_success() {
        let route = WebRoute::new("home", "/", vec![Method::Get, Method::Post], Box::new(|| Box::new(HomeController::new(Box::new(HtmlView {}))))).unwrap();

        assert_eq!(route.name, "home");
        assert_eq!(route.path, "/");
        assert_eq!(route.methods, vec![Method::Get, Method::Post]);
    }
}
