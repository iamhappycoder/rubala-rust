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

pub type ControllerConstructor = Box<dyn Fn() -> Box<dyn Controller>>;

use crate::framework ::response::Response;

pub trait Controller {
    fn run(&self) -> Box<dyn Response>;
}