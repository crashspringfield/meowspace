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

    let status = response.status();
    match status {
        Status::Ok => check_cat_response(response),
        Status::UnprocessableEntity => check_cat_validation_errors(response),
        _ => panic!("Got status: {}", status),
    }
}

#[test]
fn test_register_with_duplicate_email() {
    let client = test_client();
    register(client, "clone", EMAIL, PASSWORD);

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

    assert_eq!(response.status(), Status::UnprocessableEntity);

    let value = response_json_value(response);
    let error = value
        .get("errors")
        .and_then(|errors| errors.get("email"))
        .and_then(|errors| errors.get(0))
        .and_then(|error| error.as_str());

    assert_eq!(error, Some("has already been taken"))
}

#[test]
fn test_login() {
    let client = test_client();
    let response = &mut client
        .post("/api/cats/login")
        .header(ContentType::JSON)
        .body(json_string!({
            "cat": {
                "email": EMAIL,
                "password": PASSWORD
            }
        }))
        .dispatch();

    let value = response_json_value(response);
    value
        .get("cat")
        .expect("must have a 'cat' field")
        .get("token")
        .expect("cat has token")
        .as_str()
        .expect("token must be a string");
}

#[test]
fn bunk_login() {
    let client = test_client();
    let response = &mut client
        .post("/api/cats/login")
        .header(ContentType::JSON)
        .body(json_string!({
            "cat": {
                "email": EMAIL,
                "password": "pwned"
            }
        }))
        .dispatch();

    assert_eq!(response.status(), Status::UnprocessableEntity);

    let value = response_json_value(response);
    let login_error = value
        .get("errors")
        .expect("must have an 'errors' field")
        .get("email or password")
        .expect("must have 'email or password' errors")
        .get(0)
        .expect("must have non-empty 'email or password' errors")
        .as_str();

    assert_eq!(login_error, Some("is invalid"));
}

#[test]
fn test_get_cat() {
    let client = test_client();
    let token = login(&client);
    let response = &mut client
        .get("/api/cat")
        .header(token_header(token))
        .dispatch();

    check_cat_response(response);
}

#[test]
fn test_put_cat() {
    let client = test_client();
    let token = login(&client);
    let response = &mut client
        .put("/api/cat")
        .header(token_header(token))
        .header(ContentType::JSON)
        .body(json_string!({
            "cat": {
                "bio": "Holy meow"
            }
        }))
        .dispatch();

    check_cat_response(response);
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
