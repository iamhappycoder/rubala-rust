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

pub mod html_response;

pub use html_response::HtmlResponse;

pub trait Response {
    fn status_code(&self) -> u16;
    fn headers(&self) -> &Vec<String>;
    fn body(&self) -> &String;
}
