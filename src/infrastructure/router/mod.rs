pub mod method;
pub mod web_route;
pub mod web_router;

pub use method::Method;
pub use web_route::WebRoute;
pub use web_router::WebRouter;

pub trait Router {
    fn add_route(&mut self, route: WebRoute);
    fn match_route<T: AsRef<str>>(&self, path: T) -> Option<&WebRoute>;
}
