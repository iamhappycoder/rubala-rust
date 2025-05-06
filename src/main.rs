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

mod infrastructure;
mod interfaces;
mod framework;

use fastcgi;

use crate::framework::kernel::Kernel;

fn main() {
    fastcgi::run_once(|fcgi_request| {
        Kernel::boot(fcgi_request);
    });
}
