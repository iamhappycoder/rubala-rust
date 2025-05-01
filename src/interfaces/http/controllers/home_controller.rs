use super::Controller;
use crate::interfaces::http::response::{HtmlResponse, Response};

pub struct HomeController {}

impl HomeController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for HomeController {
    fn run(&self) -> Box<dyn Response> {
        Box::new(HtmlResponse::new(
            200,
            vec!["Content-Type: text/html".into()],
            "<h1>HomeController::run() v3</h1><br><a href=\"/about.fcgi\">about</> | <a href=\"/guest-book.fcgi\">guest book</>".into(),
        ))
    }
}
