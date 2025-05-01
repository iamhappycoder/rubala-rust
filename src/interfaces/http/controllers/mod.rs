pub mod about_controller;
pub mod home_controller;
pub mod guest_book_controller;

pub use about_controller::AboutController;
pub use home_controller::HomeController;
pub use guest_book_controller::GuestBookController;
use super::response::Response;

pub type ControllerConstructor = Box<dyn Fn() -> Box<dyn Controller>>;

pub trait Controller {
    fn run(&self) -> Box<dyn Response>;
}
