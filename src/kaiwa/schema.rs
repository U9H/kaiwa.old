table! {
    comments (id) {
        id -> Int4,
        page_id -> Int4,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        access_code -> Varchar,
        comment -> Text,
        created_at -> Timestamp,
    }
}

table! {
    pages (id) {
        id -> Int4,
        site_id -> Int4,
        slug -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    sites (id) {
        id -> Int4,
        domain -> Varchar,
        api_key -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(comments -> pages (page_id));
joinable!(pages -> sites (site_id));

allow_tables_to_appear_in_same_query!(
    comments,
    pages,
    sites,
);
