use std::collections::HashMap;

use super::Router;
use super::WebRoute;

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

    fn match_route<T: AsRef<str>>(&self, path: T) -> Option<&WebRoute> {
        self.routes.get(path.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use crate::interfaces::http::controllers::HomeController;
    use crate::infrastructure::router::method::Method;
    use crate::infrastructure::router::{Router, WebRoute, WebRouter};

    #[test]
    fn match_route_failure() {
        let mut router = WebRouter::new();

        router.add_route(WebRoute::new("home", "/", vec![Method::Get], Box::new(|| Box::new(HomeController::new()))).unwrap());

        let route = router.match_route("/x");

        assert!(route.is_none());
    }

    #[test]
    fn match_route_success() {
        let mut router = WebRouter::new();

        router.add_route( WebRoute::new("home", "/", vec![Method::Get], Box::new(|| Box::new(HomeController::new()))).unwrap());

        let route = router.match_route("/").unwrap();

        assert_eq!("home", route.name);
        assert_eq!("/", route.path);
    }
}
