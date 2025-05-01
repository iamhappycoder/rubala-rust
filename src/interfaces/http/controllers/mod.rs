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
