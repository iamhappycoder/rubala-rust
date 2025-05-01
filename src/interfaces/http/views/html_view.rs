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
use std::fs;
use std::path::PathBuf;
use super::View;

pub struct HtmlView {
}

impl View for HtmlView {
    fn render(&self, template_name: &String, params: Option<HashMap<String, String>>) -> Result<String, std::io::Error> {
        let mut path = PathBuf::from("templates");
        path.push(template_name);
        
        fs::read_to_string(path)
    }
}