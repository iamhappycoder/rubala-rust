use super::Controller;
use crate::interfaces::http::response::{HtmlResponse, Response};

pub struct AboutController {}

impl AboutController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for AboutController {
    fn run(&self) -> Box<dyn Response> {
        Box::new(HtmlResponse::new(
            200,
            vec!["Content-Type: text/html".into()],
            "<h1>AboutController::run() v3</h1><br><a href=\"/\">home</> | <a href=\"/guest-book.fcgi\">guest book</>".into(),
        ))
    }
}
