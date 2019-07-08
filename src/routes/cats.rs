// use crate::auth::Auth;
use crate::db::{self, cats::MewCatBoopsies};
use crate::errors::{Errors, FieldValidator};

use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct MewCat {
    cat: MewCatData,
}

#[derive(Deserialize, Validate, Debug)]
struct MewCatData {
    #[validate(length(min = "1"))]
    name: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = "8"))]
    password: Option<String>,
}

#[post("/cats", format = "json", data = "<mew_cat>")]
pub fn post_cats_register(mew_cat: Json<MewCat>, conn: db::Conn) -> Result<JsonValue, Errors> {
    let mew_cat = mew_cat.into_inner().cat;

    let mut extractor = FieldValidator::validate(&mew_cat);
    let name = extractor.extract("name", mew_cat.name);
    let email = extractor.extract("email", mew_cat.email);
    let password = extractor.extract("password", mew_cat.password);

    // extractor.check()?;

    db::cats::create(&conn, &name, &email, &password)
        .map(|cat| json!({ "cat": cat.to_cat_auth() }))
        .map_err(|error| {
            let field = match error {
                MewCatBoopsies::DuplicateEmail => "email",
                MewCatBoopsies::DuplicateName => "name"
            };
            Errors::new(&[(field, "has already been taken")])
        })
}
