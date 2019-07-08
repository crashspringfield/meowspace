#[derive(Queryable, Associations, Debug)]
pub struct Bird {
    pub id: i32,
    pub cat_id: i32,
    pub species: String,
    pub colors: String
}
