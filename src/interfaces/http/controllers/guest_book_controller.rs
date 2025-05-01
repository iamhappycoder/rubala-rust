use super::Controller;
use crate::interfaces::http::response::{HtmlResponse, Response};

pub struct GuestBookController {}

impl GuestBookController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for GuestBookController {
    fn run(&self) -> Box<dyn Response> {
        Box::new(HtmlResponse::new(
            200,
            vec!["Content-Type: text/html".into()],
            "<h1>GuestBookController::run() v3</h1><br><a href=\"/\">home</> | <a href=\"/about.fcgi\">about</>".into(),
        ))
    }
}
