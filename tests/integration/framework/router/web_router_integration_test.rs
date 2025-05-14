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

use rubala::framework::router::{Router, WebRouter};
use rubala::hashmap_s;

#[test]
fn match_path_segments_failure() {
    let actual_params = WebRouter::match_path_segments("/x", "/users/1");
    
    assert_eq!(actual_params, None);
}

#[test]
fn match_path_segments_success() {
    let actual_params = WebRouter::match_path_segments("/users/{id}/addresses/{address_id}", "/users/1/addresses/2");

    let expected_params = hashmap_s! {
        "id" => "1",
        "address_id" => "2",
    };

    assert_eq!(actual_params, Some(expected_params));
}
