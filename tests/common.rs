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
