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

use std::collections::HashMap;

pub mod html_view;

pub use html_view::HtmlView;

pub trait View {
    fn render(&self, template_name: &String, params: Option<HashMap<String, String>>) -> Result<String, std::io::Error>;
}