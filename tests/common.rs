#![allow(unused)]

use meowspace;
use rocket::http::{ContentType, Header, Status};
use rocket::local::{Client, LocalResponse};
use serde_json::Value;
use once_cell::sync::OnceCell;

pub const NAME: &'static str = "meowmers";
pub const EMAIL: &'static str = "meowmers@prrr.dev";
pub const PASSWORD: &'static str = "woof";

pub type Token = String;

/// Utility macro for turning `json!` into string
#[macro_export]
macro_rules! json_string {
    ($value:tt) => {
        serde_json::to_string(&serde_json::json!($value)).expect("Cannot stringify")
    };
}

pub fn test_client() -> &'static Client {
    static INSTANCE: OnceCell<Client> = OnceCell::new();

    INSTANCE.get_or_init(|| {
        let rocket = meowspace::rocket();
        Client::new(rocket).expect("valid rocket instance")
    })
}

/// Helper function for converting response to json value.
pub fn response_json_value(response: &mut LocalResponse) -> Value {
    let body = response.body().expect("no body");
    serde_json::from_reader(body.into_inner()).expect("can't parse value")
}

// Retrieve the token. Register user if required.
pub fn login(client: &Client) -> Token {
    try_login(client)
        .unwrap_or_else(|| {
            register(client, NAME, EMAIL, PASSWORD);
            try_login(client).expect("Cannot login")
        })
}

// Make an authorization header
pub fn token_header(token: Token) -> Header<'static> {
    Header::new("authorization", format!("Token {}", token))
}

pub fn register(client: &Client, name: &str, email: &str, password: &str) {
    let response = client
        .post("/api/cats")
        .header(ContentType::JSON)
        .body(json_string!({
            "cat": {
                "name": name,
                "email": email,
                "password": password
            }
        }))
        .dispatch();

    match response.status() {
        Status::Ok | Status::UnprocessableEntity => {} // ok
        status => panic!("Registration failed: {}", status)
    }
}

fn try_login(client: &Client) -> Option<Token> {
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

        if response.status() == Status::UnprocessableEntity {
            return None;
        }

        let value = response_json_value(response);
        let token = value
            .get("cat")
            .and_then(|cat| cat.get("token"))
            .and_then(|token| token.as_str())
            .map(String::from)
            .expect("Cannot extract token");
        Some(token)
}
