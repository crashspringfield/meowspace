mod common;

use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::LocalResponse;

#[test]
fn test_register() {
    let client = test_client();
    let response = &mut client
        .post("/api/cats")
        .header(ContentType::JSON)
        .body(json_string!({
            "cat": {
                "name": NAME,
                "email": EMAIL,
                "password": PASSWORD
            }
        }))
        .dispatch();

        println!("{:?}", &response);


    let status = response.status();
    match status {
        Status::Ok => check_cat_response(response),
        Status::UnprocessableEntity => check_cat_validation_errors(response),
        _ => panic!("Got status: {}", status),
    }
}


// Utility functions
fn check_cat_response(response: &mut LocalResponse) {
    let value = response_json_value(response);
    let cat = value.get("cat").expect("must have a 'cat' field");

    assert_eq!(
        cat.get("email").expect("must have email"),
        EMAIL
    );
    assert_eq!(
        cat.get("name").expect("must have name"),
        NAME
    );
}

fn check_cat_validation_errors(response: &mut LocalResponse) {
    let value = response_json_value(response);
    let cat_error = value
        .get("errors")
        .expect("must have an 'errors' field")
        .get("name")
        .expect("must have 'name' errors")
        .get(0)
        .expect("must have non-empty 'name' errors")
        .as_str();

    assert_eq!(cat_error, Some("has already been taken"))
}
