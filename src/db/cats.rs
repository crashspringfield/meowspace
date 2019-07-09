use crate::models::cats::Cat;
use crate::schema::cats;

use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use serde::Deserialize;

#[derive(Insertable)]
#[table_name = "cats"]
pub struct MewCat<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub hash: &'a str,
}

pub enum MewCatBoopsies {
    DuplicateEmail,
    DuplicateName,
}

impl From<Error> for MewCatBoopsies {
    fn from(err: Error) -> MewCatBoopsies {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            match info.constraint_name() {
                Some("cats_name_key") => return MewCatBoopsies::DuplicateName,
                Some("cats_email_key") => return MewCatBoopsies::DuplicateEmail,
                _ => {}
            }
        }
        panic!("Error creating cat: {:?}", err)
    }
}

pub fn create(
    conn: &PgConnection,
    name: &str,
    email: &str,
    password: &str,
) -> Result<Cat, MewCatBoopsies> {
    let hash = &scrypt_simple(password, &ScryptParams::new(14, 8, 1)).expect("hash error");

    let mew_cat = &MewCat {
        name,
        email,
        hash,
    };

    diesel::insert_into(cats::table)
        .values(mew_cat)
        .get_result::<Cat>(conn)
        .map_err(Into::into)
}

pub fn login(
    conn: &PgConnection,
    email: &str,
    password: &str
) -> Option<Cat> {
    let cat = cats::table
        .filter(cats::email.eq(email))
        .get_result::<Cat>(conn)
        .map_err(|err| eprintln!("login: {}", err))
        .ok()?;

    let matches = scrypt_check(password, &cat.hash)
        .map_err(|err| eprintln!("login -- scrypt_check: {}", err))
        .ok()?;

    if matches {
        Some(cat)
    } else {
        eprintln!("failed login for {}. Password doesn't match", email);
        None
    }
}

pub fn find(conn: &PgConnection, id: i32) -> Option<Cat> {
    cats::table
        .find(id)
        .get_result(conn)
        .map_err(|err| println!("find cat: {}", err))
        .ok()
}

pub fn all(conn: &PgConnection) -> Option<Vec<Cat>> {
    cats::table
        .order(cats::id.desc())
        .load::<Cat>(conn)
        .map_err(|err| println!("all cats: {}", err))
        .ok()
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "cats"]
pub struct UpdateCatData {
    name: Option<String>,
    email: Option<String>,
    bio: Option<String>,
    image: Option<String>,

    // hack to skip field
    #[column_name = "hash"]
    password: Option<String>,
}

pub fn update(conn: &PgConnection, id: i32, data: &UpdateCatData) -> Option<Cat> {
    let data = &UpdateCatData {
        password: None,
        ..data.clone()
    };

    diesel::update(cats::table.find(id))
        .set(data)
        .get_result(conn)
        .ok()
}
