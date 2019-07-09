use crate::auth::Auth;
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

#[derive(Deserialize)]
pub struct LoginCat {
    cat: LoginCatData
}

#[derive(Deserialize)]
struct LoginCatData {
    email: Option<String>,
    password: Option<String>
}

#[post("/cats/login", format = "json", data = "<cat>")]
pub fn post_cats_login(cat: Json<LoginCat>, conn: db::Conn) -> Result<JsonValue, Errors> {
    let cat = cat.into_inner().cat;

    let mut extractor = FieldValidator::default();
    let email = extractor.extract("email", cat.email);
    let password = extractor.extract("password", cat.password);

    extractor.check()?;

    db::cats::login(&conn, &email, &password)
        .map(|cat| json!({ "cat": cat.to_cat_auth() }))
        .ok_or_else(|| Errors::new(&[( "email or password", "is invalid" )]))
}

#[get("/cats")]
pub fn get_all_cats(conn: db::Conn) -> Option<JsonValue> {
    db::cats::all(&conn)
        .map(|cats| json!({
            "cats": cats
        }))
}

#[get("/cat")]
pub fn get_cat(auth: Auth, conn: db::Conn) -> Option<JsonValue> {
    db::cats::find(&conn, auth.id)
        .map(|cat| json!({
            "cat": cat.to_cat_auth()
        }))
}

#[derive(Deserialize)]
pub struct UpdateCat {
    cat: db::cats::UpdateCatData
}

#[put("/cat", format = "json", data = "<cat>")]
pub fn put_cat(cat: Json<UpdateCat>, auth: Auth, conn: db::Conn) -> Option<JsonValue> {
    db::cats::update(&conn, auth.id, &cat.cat)
        .map(|cat| json!({
            "cat": cat.to_cat_auth()
        }))
}
