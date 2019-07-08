use crate::auth::Auth;
use chrono::{Duration, Utc};
use serde::Serialize;

type Url = String;

#[derive(Queryable, Debug, Serialize, Clone)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<Url>,
    #[serde(skip_serializing)]
    pub hash: String,
}

#[derive(Serialize)]
pub struct CatAuth<'a> {
    name: &'a str,
    email: &'a str,
    bio: Option<&'a str>,
    image: Option<&'a str>,
    token: String,
}

#[derive(Serialize)]
pub struct Profile {
    name: String,
    bio: Option<String>,
    image: Option<String>
}

impl Cat {
    pub fn to_cat_auth(&self) -> CatAuth {
        let exp = Utc::now() + Duration::days(60);
        let token = Auth {
            id: self.id,
            name: self.name.clone(),
            exp: exp.timestamp(),
        }.token();

        CatAuth {
            name: &self.name,
            email: &self.email,
            bio: self.bio.as_ref().map(String::as_str),
            image: self.image.as_ref().map(String::as_str),
            token
        }
    }

    pub fn profile(self, following: bool) -> Profile {
        Profile {
            name: self.name,
            bio: self.bio,
            image: self.image,
        }
    }
}
