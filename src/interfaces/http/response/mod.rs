pub mod html_response;

pub use html_response::HtmlResponse;

pub trait Response {
    fn status_code(&self) -> u16;
    fn headers(&self) -> &Vec<String>;
    fn body(&self) -> &String;
}
