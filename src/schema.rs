table! {
    birds (id) {
        id -> Int4,
        species -> Varchar,
        colors -> Varchar,
        cat_id -> Nullable<Int4>,
    }
}

table! {
    cats (id) {
        id -> Int4,
        name -> Varchar,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

joinable!(birds -> cats (cat_id));

allow_tables_to_appear_in_same_query!(
    birds,
    cats,
);
