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

    println!("\n\n create \n\n", );

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
